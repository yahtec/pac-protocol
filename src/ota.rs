/// OTA (Over-The-Air) firmware update protocol definitions.
///
/// Shared between STM32 bootloader and ESP32 screen.
/// The STM32 remains SPI master even in bootloader mode.
///
/// Flow:
///   1. ESP32 downloads firmware from HTTP server, verifies SHA256
///   2. ESP32 sends safe shutdown commands, then ENTER_OTA_MODE
///   3. STM32 writes OTA metadata flag and reboots into bootloader
///   4. Bootloader sends OTA_READY, ESP32 sends firmware chunks
///   5. Bootloader verifies SHA256, reboots into new firmware
///   6. New firmware confirms boot OK (clears boot_attempt_count)

use crate::crc::crc16_modbus;

// ---------------------------------------------------------------------------
// OTA SPI frame format (reuses 256-byte SPI frames)
// ---------------------------------------------------------------------------

/// OTA frame size (same as normal SPI frames)
pub const OTA_FRAME_SIZE: usize = 256;

/// OTA message type identifier (byte 0)
pub const OTA_MSG_TYPE: u8 = 0xAA;

/// Maximum firmware payload per chunk (frame minus header and CRC)
/// Header: msg_type(1) + command(1) + seq(2) + payload_len(4) = 8 bytes
/// CRC: 2 bytes at offset 252-253
/// Padding: 2 bytes at 254-255
pub const OTA_CHUNK_DATA_SIZE: usize = 244;

/// CRC offset within OTA frame
pub const OTA_CRC_OFFSET: usize = 252;

/// Header size
pub const OTA_HEADER_SIZE: usize = 8;

// ---------------------------------------------------------------------------
// OTA commands
// ---------------------------------------------------------------------------

/// STM32→ESP: Bootloader ready, payload contains bootloader version + recovery flag
pub const OTA_CMD_READY: u8 = 0x01;
/// ESP→STM32: Start transfer, payload contains firmware_size(4) + sha256(32) + version(4)
pub const OTA_CMD_START: u8 = 0x02;
/// STM32→ESP: Acknowledge start, ready to receive chunks
pub const OTA_CMD_ACK_START: u8 = 0x03;
/// STM32→ESP: Request chunk N (seq_number = N)
pub const OTA_CMD_REQUEST_CHUNK: u8 = 0x04;
/// ESP→STM32: Chunk data (seq_number = N, payload = 244 bytes firmware data)
pub const OTA_CMD_CHUNK: u8 = 0x05;
/// STM32→ESP: Chunk N received and written OK
pub const OTA_CMD_ACK_CHUNK: u8 = 0x06;
/// STM32→ESP: Chunk N CRC error, please resend
pub const OTA_CMD_NACK_CHUNK: u8 = 0x07;
/// STM32→ESP: SHA256 verification passed, rebooting
pub const OTA_CMD_VERIFY_OK: u8 = 0x08;
/// STM32→ESP: SHA256 verification failed, restart transfer
pub const OTA_CMD_VERIFY_FAIL: u8 = 0x09;
/// ESP→STM32: Abort update (no firmware available or error)
pub const OTA_CMD_ABORT: u8 = 0x0A;
/// STM32→ESP: No update needed, booting normally
pub const OTA_CMD_NO_UPDATE: u8 = 0x0B;

// ---------------------------------------------------------------------------
// OTA metadata (stored in STM32 flash at OTA_METADATA_ADDR)
// ---------------------------------------------------------------------------

/// Flash address of OTA metadata page
pub const OTA_METADATA_ADDR: u32 = 0x0803_F000;

/// Flash address of application start (after bootloader)
pub const OTA_APP_ADDR: u32 = 0x0800_4000;

/// Bootloader size in bytes (16 KB = 8 pages of 2KB)
pub const OTA_BOOTLOADER_SIZE: u32 = 16 * 1024;

/// Maximum application size (232 KB)
pub const OTA_APP_MAX_SIZE: u32 = 232 * 1024;

/// User data flash address
pub const OTA_USER_DATA_ADDR: u32 = 0x0803_E000;

/// STM32L4 flash page size
pub const FLASH_PAGE_SIZE: u32 = 2048;

/// Magic number "OTA\0"
pub const OTA_MAGIC: u32 = 0x4F54_4100;

/// Metadata flags
pub const OTA_FLAG_UPDATE_PENDING: u32 = 1 << 0;
pub const OTA_FLAG_UPDATE_IN_PROGRESS: u32 = 1 << 1;
pub const OTA_FLAG_BOOT_VERIFIED: u32 = 1 << 2;

/// Max boot attempts before entering recovery mode
pub const OTA_MAX_BOOT_ATTEMPTS: u32 = 3;

/// Max retries per chunk before aborting
pub const OTA_MAX_CHUNK_RETRIES: u8 = 3;

/// Max full transfer retries before giving up
pub const OTA_MAX_TRANSFER_RETRIES: u8 = 3;

/// Timeout waiting for OTA_START from ESP32 (milliseconds)
pub const OTA_START_TIMEOUT_MS: u32 = 30_000;

/// Timeout waiting for a single chunk response (milliseconds)
pub const OTA_CHUNK_TIMEOUT_MS: u32 = 5_000;

/// OTA SPI frequency (can be faster than normal operation)
pub const OTA_SPI_FREQUENCY_HZ: u32 = 500_000;

/// SHA256 hash length
pub const SHA256_LEN: usize = 32;

// ---------------------------------------------------------------------------
// OTA Metadata structure (56 bytes, stored at OTA_METADATA_ADDR)
// ---------------------------------------------------------------------------

/// OTA metadata stored in flash. Total 56 bytes.
/// Layout matches the plan:
///   0x00: magic (4)
///   0x04: flags (4)
///   0x08: firmware_size (4)
///   0x0C: firmware_sha256 (32)
///   0x2C: firmware_version (4)
///   0x30: boot_attempt_count (4)
///   0x34: crc32 (4)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct OtaMetadata {
    pub magic: u32,
    pub flags: u32,
    pub firmware_size: u32,
    pub firmware_sha256: [u8; SHA256_LEN],
    pub firmware_version: u32,
    pub boot_attempt_count: u32,
    pub crc32: u32,
}

impl OtaMetadata {
    pub const SIZE: usize = 56;

    pub const fn empty() -> Self {
        Self {
            magic: 0,
            flags: 0,
            firmware_size: 0,
            firmware_sha256: [0u8; SHA256_LEN],
            firmware_version: 0,
            boot_attempt_count: 0,
            crc32: 0,
        }
    }

    /// Check if the metadata has a valid magic number
    pub fn is_valid(&self) -> bool {
        self.magic == OTA_MAGIC && self.verify_crc()
    }

    /// Check if an update is pending
    pub fn update_pending(&self) -> bool {
        self.flags & OTA_FLAG_UPDATE_PENDING != 0
    }

    /// Check if an update is in progress
    pub fn update_in_progress(&self) -> bool {
        self.flags & OTA_FLAG_UPDATE_IN_PROGRESS != 0
    }

    /// Check if boot has been verified
    pub fn boot_verified(&self) -> bool {
        self.flags & OTA_FLAG_BOOT_VERIFIED != 0
    }

    /// Check if we should enter recovery mode
    pub fn needs_recovery(&self) -> bool {
        self.boot_attempt_count >= OTA_MAX_BOOT_ATTEMPTS
    }

    /// Serialize to bytes (little-endian, matches ARM native layout)
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        buf[0..4].copy_from_slice(&self.magic.to_le_bytes());
        buf[4..8].copy_from_slice(&self.flags.to_le_bytes());
        buf[8..12].copy_from_slice(&self.firmware_size.to_le_bytes());
        buf[12..44].copy_from_slice(&self.firmware_sha256);
        buf[44..48].copy_from_slice(&self.firmware_version.to_le_bytes());
        buf[48..52].copy_from_slice(&self.boot_attempt_count.to_le_bytes());
        buf[52..56].copy_from_slice(&self.crc32.to_le_bytes());
        buf
    }

    /// Deserialize from bytes
    pub fn from_bytes(buf: &[u8; Self::SIZE]) -> Self {
        let mut sha = [0u8; SHA256_LEN];
        sha.copy_from_slice(&buf[12..44]);
        Self {
            magic: u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]),
            flags: u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]),
            firmware_size: u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]),
            firmware_sha256: sha,
            firmware_version: u32::from_le_bytes([buf[44], buf[45], buf[46], buf[47]]),
            boot_attempt_count: u32::from_le_bytes([buf[48], buf[49], buf[50], buf[51]]),
            crc32: u32::from_le_bytes([buf[52], buf[53], buf[54], buf[55]]),
        }
    }

    /// Compute CRC32 over bytes 0-51 (everything except the crc32 field itself)
    pub fn compute_crc(&self) -> u32 {
        let buf = self.to_bytes();
        crc32(&buf[..52])
    }

    /// Verify the CRC32
    pub fn verify_crc(&self) -> bool {
        self.crc32 == self.compute_crc()
    }

    /// Set the CRC32 field from computed value
    pub fn update_crc(&mut self) {
        self.crc32 = self.compute_crc();
    }
}

// ---------------------------------------------------------------------------
// OTA frame builder/parser
// ---------------------------------------------------------------------------

/// Build an OTA SPI frame
/// Returns a 256-byte frame with header, payload, and CRC-16
pub fn ota_build_frame(command: u8, seq: u16, payload: &[u8]) -> [u8; OTA_FRAME_SIZE] {
    let mut frame = [0u8; OTA_FRAME_SIZE];
    frame[0] = OTA_MSG_TYPE;
    frame[1] = command;
    frame[2] = (seq >> 8) as u8;
    frame[3] = (seq & 0xFF) as u8;
    let len = payload.len().min(OTA_CHUNK_DATA_SIZE);
    frame[4] = ((len >> 24) & 0xFF) as u8;
    frame[5] = ((len >> 16) & 0xFF) as u8;
    frame[6] = ((len >> 8) & 0xFF) as u8;
    frame[7] = (len & 0xFF) as u8;
    frame[OTA_HEADER_SIZE..OTA_HEADER_SIZE + len].copy_from_slice(&payload[..len]);

    let crc = crc16_modbus(&frame[..OTA_CRC_OFFSET]);
    frame[OTA_CRC_OFFSET] = (crc >> 8) as u8;
    frame[OTA_CRC_OFFSET + 1] = (crc & 0xFF) as u8;
    frame
}

/// Parse OTA frame header. Returns (command, seq, payload_len) or None if invalid.
pub fn ota_parse_frame(frame: &[u8; OTA_FRAME_SIZE]) -> Option<(u8, u16, usize)> {
    if frame[0] != OTA_MSG_TYPE {
        return None;
    }
    // Verify CRC
    let crc = crc16_modbus(&frame[..OTA_CRC_OFFSET]);
    let crc_hi = (crc >> 8) as u8;
    let crc_lo = (crc & 0xFF) as u8;
    if crc_hi != frame[OTA_CRC_OFFSET] || crc_lo != frame[OTA_CRC_OFFSET + 1] {
        return None;
    }
    let command = frame[1];
    let seq = ((frame[2] as u16) << 8) | (frame[3] as u16);
    let payload_len = ((frame[4] as usize) << 24)
        | ((frame[5] as usize) << 16)
        | ((frame[6] as usize) << 8)
        | (frame[7] as usize);
    Some((command, seq, payload_len.min(OTA_CHUNK_DATA_SIZE)))
}

/// Get payload slice from a parsed OTA frame
pub fn ota_frame_payload(frame: &[u8; OTA_FRAME_SIZE], len: usize) -> &[u8] {
    &frame[OTA_HEADER_SIZE..OTA_HEADER_SIZE + len.min(OTA_CHUNK_DATA_SIZE)]
}

// ---------------------------------------------------------------------------
// OTA_READY payload (STM32 → ESP32)
// ---------------------------------------------------------------------------

/// Build OTA_READY payload: bootloader_version(4) + recovery_flag(1)
pub fn ota_build_ready_payload(bootloader_version: u32, recovery: bool) -> [u8; 5] {
    let mut p = [0u8; 5];
    p[0..4].copy_from_slice(&bootloader_version.to_be_bytes());
    p[4] = if recovery { 1 } else { 0 };
    p
}

/// Parse OTA_READY payload
pub fn ota_parse_ready_payload(payload: &[u8]) -> (u32, bool) {
    if payload.len() < 5 {
        return (0, false);
    }
    let version = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]);
    let recovery = payload[4] != 0;
    (version, recovery)
}

// ---------------------------------------------------------------------------
// OTA_START payload (ESP32 → STM32)
// ---------------------------------------------------------------------------

/// Build OTA_START payload: firmware_size(4) + sha256(32) + version(4) = 40 bytes
pub fn ota_build_start_payload(size: u32, sha256: &[u8; SHA256_LEN], version: u32) -> [u8; 40] {
    let mut p = [0u8; 40];
    p[0..4].copy_from_slice(&size.to_be_bytes());
    p[4..36].copy_from_slice(sha256);
    p[36..40].copy_from_slice(&version.to_be_bytes());
    p
}

/// Parse OTA_START payload
pub fn ota_parse_start_payload(payload: &[u8]) -> Option<(u32, [u8; SHA256_LEN], u32)> {
    if payload.len() < 40 {
        return None;
    }
    let size = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]);
    let mut sha = [0u8; SHA256_LEN];
    sha.copy_from_slice(&payload[4..36]);
    let version = u32::from_be_bytes([payload[36], payload[37], payload[38], payload[39]]);
    Some((size, sha, version))
}

// ---------------------------------------------------------------------------
// CRC-32 (for metadata integrity)
// ---------------------------------------------------------------------------

/// Simple CRC-32 (IEEE 802.3 polynomial).
/// Used for OTA metadata integrity check.
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

// ---------------------------------------------------------------------------
// Utility: compute number of chunks for a given firmware size
// ---------------------------------------------------------------------------

/// Calculate the number of OTA chunks needed for a firmware of the given size
pub fn ota_chunk_count(firmware_size: u32) -> u16 {
    let count = (firmware_size as usize + OTA_CHUNK_DATA_SIZE - 1) / OTA_CHUNK_DATA_SIZE;
    count as u16
}

/// Calculate byte offset for a given chunk index
pub fn ota_chunk_offset(chunk_index: u16) -> u32 {
    chunk_index as u32 * OTA_CHUNK_DATA_SIZE as u32
}

/// Calculate payload size for the last chunk (may be smaller than OTA_CHUNK_DATA_SIZE)
pub fn ota_last_chunk_size(firmware_size: u32) -> usize {
    let remainder = firmware_size as usize % OTA_CHUNK_DATA_SIZE;
    if remainder == 0 { OTA_CHUNK_DATA_SIZE } else { remainder }
}

// ---------------------------------------------------------------------------
// SPI input register command for triggering OTA from application
// ---------------------------------------------------------------------------

/// Byte offset in SPI Type 2 (Input Registers) for the OTA command.
/// ESP32 writes this to signal the STM32 application to enter OTA mode.
/// Value: 0x00 = no action, 0x01 = enter OTA mode
pub const SPI_INPUT_OTA_CMD_OFFSET: usize = 240;
/// Command value to request OTA mode entry
pub const SPI_OTA_ENTER_CMD: u8 = 0x01;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_roundtrip() {
        let mut meta = OtaMetadata {
            magic: OTA_MAGIC,
            flags: OTA_FLAG_UPDATE_PENDING,
            firmware_size: 159744,
            firmware_sha256: [0xAB; SHA256_LEN],
            firmware_version: 0x0003_1200,
            boot_attempt_count: 0,
            crc32: 0,
        };
        meta.update_crc();

        let bytes = meta.to_bytes();
        let parsed = OtaMetadata::from_bytes(&bytes);

        assert!(parsed.is_valid());
        assert!(parsed.update_pending());
        assert!(!parsed.update_in_progress());
        assert_eq!(parsed.firmware_size, 159744);
        assert_eq!(parsed.firmware_sha256, [0xAB; SHA256_LEN]);
    }

    #[test]
    fn test_metadata_invalid_magic() {
        let meta = OtaMetadata::empty();
        assert!(!meta.is_valid());
    }

    #[test]
    fn test_metadata_corrupted_crc() {
        let mut meta = OtaMetadata {
            magic: OTA_MAGIC,
            flags: 0,
            firmware_size: 100,
            firmware_sha256: [0; SHA256_LEN],
            firmware_version: 1,
            boot_attempt_count: 0,
            crc32: 0,
        };
        meta.update_crc();
        meta.firmware_size = 999; // corrupt after CRC
        assert!(!meta.verify_crc());
    }

    #[test]
    fn test_frame_roundtrip() {
        let payload = [0x42u8; 100];
        let frame = ota_build_frame(OTA_CMD_CHUNK, 7, &payload);
        let parsed = ota_parse_frame(&frame);
        assert!(parsed.is_some());
        let (cmd, seq, len) = parsed.unwrap();
        assert_eq!(cmd, OTA_CMD_CHUNK);
        assert_eq!(seq, 7);
        assert_eq!(len, 100);
        assert_eq!(ota_frame_payload(&frame, len)[..100], payload[..]);
    }

    #[test]
    fn test_frame_crc_corruption() {
        let frame = ota_build_frame(OTA_CMD_READY, 0, &[1, 2, 3]);
        let mut corrupted = frame;
        corrupted[10] ^= 0xFF;
        assert!(ota_parse_frame(&corrupted).is_none());
    }

    #[test]
    fn test_ready_payload() {
        let p = ota_build_ready_payload(1, true);
        let (ver, rec) = ota_parse_ready_payload(&p);
        assert_eq!(ver, 1);
        assert!(rec);
    }

    #[test]
    fn test_start_payload() {
        let sha = [0xDE; SHA256_LEN];
        let p = ota_build_start_payload(159744, &sha, 0x318);
        let parsed = ota_parse_start_payload(&p);
        assert!(parsed.is_some());
        let (size, s, ver) = parsed.unwrap();
        assert_eq!(size, 159744);
        assert_eq!(s, sha);
        assert_eq!(ver, 0x318);
    }

    #[test]
    fn test_chunk_count() {
        // 156 KB = 159744 bytes / 244 = 654.7 → 655 chunks
        assert_eq!(ota_chunk_count(159744), 655);
        // Exact multiple
        assert_eq!(ota_chunk_count(244), 1);
        assert_eq!(ota_chunk_count(488), 2);
        assert_eq!(ota_chunk_count(245), 2);
    }

    #[test]
    fn test_last_chunk_size() {
        assert_eq!(ota_last_chunk_size(488), 244); // exact multiple
        assert_eq!(ota_last_chunk_size(245), 1);
        assert_eq!(ota_last_chunk_size(159744), 159744 % 244); // 16 bytes
    }

    #[test]
    fn test_crc32_known() {
        // CRC-32 of "123456789" = 0xCBF43926
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn test_recovery_detection() {
        let mut meta = OtaMetadata {
            magic: OTA_MAGIC,
            flags: OTA_FLAG_BOOT_VERIFIED,
            firmware_size: 100,
            firmware_sha256: [0; SHA256_LEN],
            firmware_version: 1,
            boot_attempt_count: 3,
            crc32: 0,
        };
        meta.update_crc();
        assert!(meta.needs_recovery());

        meta.boot_attempt_count = 2;
        meta.update_crc();
        assert!(!meta.needs_recovery());
    }
}
