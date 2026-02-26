/// SPI protocol definitions for PAC Hybride system.
/// Full-duplex 256-byte frames at 500kHz between STM32 (master) and ESP32 (slave).
///
/// Frame layout: 254 bytes data + 2 bytes CRC-16 Modbus (big-endian at bytes 254-255).
/// Three message types rotate in cycles:
///   - Type 1: Output registers (STM32 read sensors → ESP32)
///   - Type 2: Input registers (ESP32 commands → STM32)
///   - Type 3: WiFi data (SSID, config, versions, faults)

use crate::crc::crc16_modbus;

// Frame constants
pub const SPI_FRAME_SIZE: usize = 256;
pub const SPI_CRC_OFFSET: usize = 254;
pub const SPI_DATA_PAYLOAD: usize = 254;
pub const SPI_FREQUENCY_HZ: u32 = 500_000;

// Message types (word 0 of the frame, big-endian at bytes 0-1)
pub const SPI_MSG_OUTPUT_REGS: u16 = 1;
pub const SPI_MSG_INPUT_REGS: u16 = 2;
pub const SPI_MSG_WIFI: u16 = 3;

// Number of SPI cycles per rotation
pub const SPI_CYCLE_COUNT_STM32: u8 = 3;
pub const SPI_CYCLE_COUNT_ESP32: u8 = 2;

// WiFi frame byte offsets (message type 3, STM32 → ESP32)
pub const SPI_WIFI_SSID_OFFSET: usize = 2;
pub const SPI_WIFI_SSID_LEN: usize = 32;
pub const SPI_WIFI_PAC_ID_OFFSET: usize = 34;
pub const SPI_WIFI_CONFIG_OFFSET: usize = 36;
pub const SPI_WIFI_FAULT_OFFSET: usize = 54;
pub const SPI_WIFI_BOILER_FAULT_OFFSET: usize = 60;
pub const SPI_WIFI_TEMP_MAX_OFFSET: usize = 62;
pub const SPI_WIFI_COEF_WF_OFFSET: usize = 64;
pub const SPI_WIFI_MAX_PRESSURE_OFFSET: usize = 66;
pub const SPI_WIFI_VERSION_STM32_OFFSET: usize = 68;
pub const SPI_WIFI_VERSION_SCREEN_OFFSET: usize = 72;
pub const SPI_VERIFY_OFFSET: usize = 200;

// Register data offset (message types 1 and 2)
pub const SPI_DATA_START_OFFSET: usize = 2;
pub const SPI_MAX_REGISTERS: usize = 100;

/// Pack a u16 value into a byte buffer at the given offset (big-endian).
#[inline]
pub fn spi_pack_u16(buf: &mut [u8], offset: usize, value: u16) {
    buf[offset] = (value >> 8) as u8;
    buf[offset + 1] = (value & 0xFF) as u8;
}

/// Unpack a u16 value from a byte buffer at the given offset (big-endian).
#[inline]
pub fn spi_unpack_u16(buf: &[u8], offset: usize) -> u16 {
    ((buf[offset] as u16) << 8) | (buf[offset + 1] as u16)
}

/// Pack an i16 value into a byte buffer at the given offset (big-endian).
#[inline]
pub fn spi_pack_i16(buf: &mut [u8], offset: usize, value: i16) {
    spi_pack_u16(buf, offset, value as u16);
}

/// Unpack an i16 value from a byte buffer at the given offset (big-endian).
#[inline]
pub fn spi_unpack_i16(buf: &[u8], offset: usize) -> i16 {
    spi_unpack_u16(buf, offset) as i16
}

/// Validate CRC-16 Modbus on a SPI frame.
/// Frame must be at least 256 bytes. CRC at bytes 254-255 (big-endian).
pub fn spi_validate_crc(frame: &[u8]) -> bool {
    if frame.len() < SPI_FRAME_SIZE {
        return false;
    }
    let crc = crc16_modbus(&frame[..SPI_CRC_OFFSET]);
    let crc_hi = (crc >> 8) as u8;
    let crc_lo = (crc & 0xFF) as u8;
    crc_hi == frame[SPI_CRC_OFFSET] && crc_lo == frame[SPI_CRC_OFFSET + 1]
}

/// Compute and append CRC-16 Modbus to a SPI frame.
/// Frame must be at least 256 bytes. CRC written at bytes 254-255 (big-endian).
pub fn spi_append_crc(frame: &mut [u8]) {
    let crc = crc16_modbus(&frame[..SPI_CRC_OFFSET]);
    frame[SPI_CRC_OFFSET] = (crc >> 8) as u8;
    frame[SPI_CRC_OFFSET + 1] = (crc & 0xFF) as u8;
}

/// Set the message type in the frame header (bytes 0-1).
#[inline]
pub fn spi_set_msg_type(frame: &mut [u8], msg_type: u16) {
    spi_pack_u16(frame, 0, 0);
    spi_pack_u16(frame, 0, msg_type);
}

/// Set the verify echo at offset 200-201 (must match message type).
#[inline]
pub fn spi_set_verify(frame: &mut [u8], msg_type: u16) {
    frame[SPI_VERIFY_OFFSET] = 0;
    frame[SPI_VERIFY_OFFSET + 1] = msg_type as u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spi_pack_unpack_u16() {
        let mut buf = [0u8; 4];
        spi_pack_u16(&mut buf, 0, 0x1234);
        assert_eq!(buf[0], 0x12);
        assert_eq!(buf[1], 0x34);
        assert_eq!(spi_unpack_u16(&buf, 0), 0x1234);
    }

    #[test]
    fn test_spi_pack_unpack_i16_negative() {
        let mut buf = [0u8; 4];
        spi_pack_i16(&mut buf, 0, -256);
        assert_eq!(spi_unpack_i16(&buf, 0), -256);
    }

    #[test]
    fn test_spi_pack_unpack_i16_positive() {
        let mut buf = [0u8; 4];
        spi_pack_i16(&mut buf, 0, 1234);
        assert_eq!(spi_unpack_i16(&buf, 0), 1234);
    }

    #[test]
    fn test_spi_crc_roundtrip() {
        let mut frame = [0u8; 256];
        frame[0] = 0x00;
        frame[1] = 0x01;
        frame[2] = 0xAB;
        frame[50] = 0xCD;
        spi_append_crc(&mut frame);
        assert!(spi_validate_crc(&frame));
    }

    #[test]
    fn test_spi_crc_corruption_detected() {
        let mut frame = [0u8; 256];
        frame[0] = 0x00;
        frame[1] = 0x01;
        spi_append_crc(&mut frame);
        frame[10] = 0xFF; // corrupt one byte
        assert!(!spi_validate_crc(&frame));
    }

    #[test]
    fn test_spi_crc_too_short() {
        let frame = [0u8; 100];
        assert!(!spi_validate_crc(&frame));
    }

    #[test]
    fn test_spi_msg_type_constants() {
        assert_eq!(SPI_MSG_OUTPUT_REGS, 1);
        assert_eq!(SPI_MSG_INPUT_REGS, 2);
        assert_eq!(SPI_MSG_WIFI, 3);
    }
}
