/// Data conversion functions shared across all PAC firmwares.
/// Single source of truth to prevent byte-order or scaling mismatches.

/// Decode two CAN bytes (big-endian) into u16.
#[inline]
pub fn data_to_u16(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

/// Decode two CAN bytes (big-endian) into i16.
#[inline]
pub fn data_to_i16(high: u8, low: u8) -> i16 {
    data_to_u16(high, low) as i16
}

/// Decode two CAN bytes (big-endian) into f32, dividing by 10 (unscale).
/// Use for registers with `scaled: true`.
#[inline]
pub fn data_to_f32_scaled(high: u8, low: u8) -> f32 {
    data_to_i16(high, low) as f32 / 10.0
}

/// Encode a float value into two CAN bytes (big-endian).
/// If `scaled` is true, multiplies by 10 before encoding.
#[inline]
pub fn f32_to_can_bytes(value: f32, scaled: bool) -> (u8, u8) {
    let wire_value = if scaled { value * 10.0 } else { value };
    let clamped = wire_value.clamp(i16::MIN as f32, i16::MAX as f32);
    let int_val = clamped as i16;
    let high = ((int_val >> 8) & 0xFF) as u8;
    let low = (int_val & 0xFF) as u8;
    (high, low)
}

/// Encode an i16 value into two CAN bytes (big-endian).
#[inline]
pub fn i16_to_can_bytes(value: i16) -> (u8, u8) {
    let high = ((value >> 8) & 0xFF) as u8;
    let low = (value & 0xFF) as u8;
    (high, low)
}

/// Extract individual bits from a u16 fault/status code.
/// Returns array of 16 booleans, index 0 = LSB.
pub fn u16_to_bits(code: u16) -> [bool; 16] {
    let mut bits = [false; 16];
    let mut i = 0;
    while i < 16 {
        bits[i] = (code >> i) & 1 != 0;
        i += 1;
    }
    bits
}

/// Pack SSID bytes (up to 32) into 7 CAN frame payloads of 5 bytes each.
/// Returns array of 7 arrays of 5 bytes.
pub fn ssid_to_can_frames(ssid: &[u8; 32]) -> [[u8; 5]; 7] {
    let mut frames = [[0u8; 5]; 7];
    let mut i = 0;
    while i < 7 {
        let base = i * 5;
        let mut j = 0;
        while j < 5 && base + j < 32 {
            frames[i][j] = ssid[base + j];
            j += 1;
        }
        i += 1;
    }
    frames
}

/// Unpack 7 CAN frame payloads of 5 bytes each into a 32-byte SSID buffer.
pub fn can_frames_to_ssid(frames: &[[u8; 5]; 7]) -> [u8; 32] {
    let mut ssid = [0u8; 32];
    let mut i = 0;
    while i < 7 {
        let base = i * 5;
        let mut j = 0;
        while j < 5 && base + j < 32 {
            ssid[base + j] = frames[i][j];
            j += 1;
        }
        i += 1;
    }
    ssid
}

/// Convert i16 to f32 with division by scale factor.
/// Used for converting raw sensor registers to display values.
/// Example: `i16_to_f32(245, 10.0)` → `24.5`
#[inline]
pub fn i16_to_f32(val: i16, scale: f32) -> f32 {
    val as f32 / scale
}

/// Convert f32 to i16 with clamping and optional scale multiplication.
/// Safe conversion that clamps to i16 range.
#[inline]
pub fn f32_to_i16(val: f32, scale: f32) -> i16 {
    let scaled = val * scale;
    if scaled > i16::MAX as f32 {
        i16::MAX
    } else if scaled < i16::MIN as f32 {
        i16::MIN
    } else {
        scaled as i16
    }
}
