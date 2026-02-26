/// CRC-16 implementation for PAC communication.
/// Used for:
///   - SPI frames between STM32 and ESP32 (polynomial 0xA001, init 0xFFFF)
///   - CAN frames (A7 improvement): lightweight CRC in unused bytes 5-6

/// CRC-16 Modbus (polynomial 0xA001, init 0xFFFF).
/// Used by ESP32 SPI protocol for frame validation.
pub fn crc16_modbus(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}

/// Lightweight CRC-8 for CAN frames.
/// Uses the payload bytes [0..5] and writes CRC into byte[5] of the 8-byte frame.
/// This uses polynomial 0x07 (CRC-8/CCITT), providing error detection on the CAN application layer.
pub fn crc8_can(data: &[u8; 5]) -> u8 {
    let mut crc: u8 = 0x00;
    for &byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if crc & 0x80 != 0 {
                crc = (crc << 1) ^ 0x07;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

/// Verify CRC-8 on a received CAN frame.
/// `frame` is the full 8-byte CAN data. CRC is in byte[5], computed over bytes[0..5].
pub fn verify_crc8_can(frame: &[u8]) -> bool {
    if frame.len() < 6 {
        return false;
    }
    let expected = crc8_can(&[frame[0], frame[1], frame[2], frame[3], frame[4]]);
    expected == frame[5]
}

/// Build a CAN frame with CRC-8.
/// Takes the 5 meaningful bytes and returns an 8-byte frame with CRC in byte[5].
pub fn build_can_frame_with_crc(data: &[u8; 5]) -> [u8; 8] {
    let crc = crc8_can(data);
    [data[0], data[1], data[2], data[3], data[4], crc, 0x00, 0x00]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc16_modbus_known_value() {
        // "123456789" → CRC-16/Modbus = 0x4B37
        let data = b"123456789";
        assert_eq!(crc16_modbus(data), 0x4B37);
    }

    #[test]
    fn test_crc8_can_roundtrip() {
        let payload = [0x10, 0x00, 0x01, 0x00, 0xFA];
        let frame = build_can_frame_with_crc(&payload);
        assert!(verify_crc8_can(&frame));
    }

    #[test]
    fn test_crc8_can_corruption_detected() {
        let payload = [0x10, 0x00, 0x01, 0x00, 0xFA];
        let mut frame = build_can_frame_with_crc(&payload);
        frame[3] ^= 0x01; // corrupt one byte
        assert!(!verify_crc8_can(&frame));
    }
}
