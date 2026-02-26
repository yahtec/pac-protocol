/// Modbus RTU protocol definitions for PAC Hybride system.
///
/// The STM32 acts as both:
///   - Slave (address 0x50): exposes read registers 10-76 and write registers 100-124
///   - Master: queries inverter (Sanhua SD2), boiler pump (Grundfos Para Maxo), gas sensors

use crate::crc::crc16_modbus;

// ─── Function codes ──────────────────────────────────────────────────────────

pub const FC_READ_HOLDING_REGISTERS: u8 = 0x03;
pub const FC_READ_INPUT_REGISTERS: u8 = 0x04;
pub const FC_WRITE_SINGLE_REGISTER: u8 = 0x06;
pub const FC_WRITE_MULTIPLE_REGISTERS: u8 = 0x10;

// ─── PAC Slave ───────────────────────────────────────────────────────────────

pub const SLAVE_ADDRESS: u8 = 0x50;

// Read register addresses (10-76)
pub const REG_MACHINE_TYPE: u16 = 10;
pub const REG_BITS_STATES1: u16 = 11;
pub const REG_BITS_STATES2: u16 = 12;
pub const REG_BITS_STATES3: u16 = 13;
pub const REG_T_INPUT_BOILER: u16 = 14;
pub const REG_T_OUTPUT_BOILER: u16 = 15;
pub const REG_T_SMOKE_BOILER: u16 = 16;
pub const REG_T_INPUT_PAC: u16 = 17;
pub const REG_T_BP_PAC: u16 = 18;
pub const REG_T_HP_PAC: u16 = 19;
pub const REG_T_TANK_ECS: u16 = 20;
pub const REG_T_OUTDOOR: u16 = 21;
pub const REG_T_CONDENSATION: u16 = 22;
pub const REG_T_EVAPORATION: u16 = 23;
pub const REG_T_OVERHEATING: u16 = 24;
pub const REG_T_OVERCOOLING: u16 = 25;
pub const REG_WATERFLOW_BOILER: u16 = 26;
pub const REG_WATERFLOW_ECS: u16 = 27;
pub const REG_FAN_SPEED: u16 = 28;
pub const REG_WATER_PRESSURE: u16 = 29;
pub const REG_AIR_PRESSURE: u16 = 30;
pub const REG_FAN_AIR_PRESSURE: u16 = 31;
pub const REG_PRESSURE_HP: u16 = 32;
pub const REG_PRESSURE_BP: u16 = 33;
pub const REG_POSITION_DETENDEUR: u16 = 34;
pub const REG_PUMP_BOILER_DP: u16 = 35;
pub const REG_PUMP_BOILER_WF: u16 = 36;
pub const REG_PUMP_BOILER_ENERGY: u16 = 37;
pub const REG_PUMP_BOILER_POWER: u16 = 38;
pub const REG_PUMP_BOILER_TIME: u16 = 39;
pub const REG_PUMP_BOILER_SPEED: u16 = 40;
pub const REG_PUMP_BOILER_MODE: u16 = 41;
pub const REG_PUMP_ECS_DP: u16 = 42;
pub const REG_PUMP_ECS_WF: u16 = 43;
pub const REG_PUMP_ECS_ENERGY: u16 = 44;
pub const REG_PUMP_ECS_POWER: u16 = 45;
pub const REG_PUMP_ECS_TIME: u16 = 46;
pub const REG_PUMP_ECS_SPEED: u16 = 47;
pub const REG_PUMP_ECS_MODE: u16 = 48;
pub const REG_PUMP_LOOP_DP: u16 = 49;
pub const REG_PUMP_LOOP_WF: u16 = 50;
pub const REG_PUMP_LOOP_ENERGY: u16 = 51;
pub const REG_PUMP_LOOP_POWER: u16 = 52;
pub const REG_PUMP_LOOP_TIME: u16 = 53;
pub const REG_PUMP_LOOP_SPEED: u16 = 54;
pub const REG_PUMP_LOOP_MODE: u16 = 55;
pub const REG_COMPRESSOR_STATUS: u16 = 56;
pub const REG_COMPRESSOR_OUT_FREQ: u16 = 57;
pub const REG_COMPRESSOR_OUT_VOLT: u16 = 58;
pub const REG_COMPRESSOR_OUT_CURRENT: u16 = 59;
pub const REG_COMPRESSOR_OUT_POWER: u16 = 60;
pub const REG_COMPRESSOR_IN_VOLT: u16 = 61;
pub const REG_COMPRESSOR_IN_CURRENT: u16 = 62;
pub const REG_COMPRESSOR_IN_POWER: u16 = 63;
pub const REG_INVERTER_IPM_TEMP: u16 = 64;
pub const REG_PFC_MODULE_TEMP: u16 = 65;
pub const REG_PCP_TEMP: u16 = 66;
pub const REG_COMPRESSOR_FAULT0: u16 = 67;
pub const REG_COMPRESSOR_FAULT1: u16 = 68;
pub const REG_COMPRESSOR_FAULT2: u16 = 69;
pub const REG_FAULT_CURRENT: u16 = 70;
pub const REG_STATUS_BOILER: u16 = 71;
pub const REG_STATUS_PAC: u16 = 72;
pub const REG_SOURCE_AVAILABLE: u16 = 73;
pub const REG_FAN_SPEED_PAC: u16 = 74;
pub const REG_COMPRESSOR_IN_FREQ: u16 = 75;
pub const REG_ANTI_SHORT_CYCLE: u16 = 76;

pub const READ_REG_FIRST: u16 = 10;
pub const READ_REG_LAST: u16 = 76;
pub const READ_REG_COUNT: u16 = READ_REG_LAST - READ_REG_FIRST + 1;

// Write register addresses (100-124)
pub const WREG_PROTOCOL_VERSION: u16 = 100;
pub const WREG_BITS_STATE1: u16 = 101;
pub const WREG_BITS_STATE2: u16 = 102;
pub const WREG_REQ_BOILER_PWR: u16 = 103;
pub const WREG_REQ_TEMPERATURE_OUT: u16 = 104;
pub const WREG_REQ_INV_DIR: u16 = 105;
pub const WREG_REQ_WATERFLOW: u16 = 106;
pub const WREG_BOILER_SPEED_MIN: u16 = 107;
pub const WREG_BOILER_SPEED_MAX: u16 = 108;
pub const WREG_BOILER_SPEED_PREVENT: u16 = 109;
pub const WREG_FAN_SPEED_PAC: u16 = 110;
pub const WREG_COMPRESSOR_FREQ: u16 = 111;
pub const WREG_EXPANSION_VALVE_POS: u16 = 112;
pub const WREG_PUMP_FORCE_SPEED: u16 = 113;
pub const WREG_PUMP_ECS_FORCE_SPEED: u16 = 114;
pub const WREG_T_MAX_OUT: u16 = 115;
pub const WREG_P: u16 = 116;
pub const WREG_I: u16 = 117;
pub const WREG_D: u16 = 118;
pub const WREG_COEF_WATERFLOW: u16 = 119;
pub const WREG_P_WATER_MAX: u16 = 120;
pub const WREG_T_GLYCOL: u16 = 121;
pub const WREG_PWR_MAX: u16 = 122;
pub const WREG_O2: u16 = 123;
pub const WREG_NOX: u16 = 124;

pub const WRITE_REG_FIRST: u16 = 100;
pub const WRITE_REG_LAST: u16 = 124;
pub const WRITE_REG_COUNT: u16 = WRITE_REG_LAST - WRITE_REG_FIRST + 1;

// ─── Master device: Inverter Sanhua SD2 ──────────────────────────────────────

pub mod inverter_sd2 {
    pub const READ_REG_START: u16 = 0;
    pub const READ_REG_COUNT: u16 = 29;
    pub const WREG_FREQUENCY: u16 = 0;
    pub const WREG_DIRECTION: u16 = 3;
}

// ─── Master device: Boiler Pump Grundfos Para Maxo ───────────────────────────

pub mod pump_para_maxo {
    pub const READ_REG_START: u16 = 1;
    pub const READ_REG_COUNT: u16 = 7;
    pub const WREG_SETPOINT: u16 = 1;
    pub const WREG_CONTROL_MODE: u16 = 40;
    pub const WREG_STOP_START: u16 = 42;
}

// ─── Master device: Gas Sensor ───────────────────────────────────────────────

pub mod gas_sensor {
    pub const READ_REG_START: u16 = 0x0110;
    pub const READ_REG_COUNT: u16 = 6;
    pub const WREG_ZERO_CALIBRATION: u16 = 0x0101;
}

// ─── CRC helpers ─────────────────────────────────────────────────────────────

/// Validate CRC-16 Modbus on a Modbus RTU frame.
/// CRC is the last 2 bytes in standard Modbus order (low byte first).
pub fn modbus_validate_crc(frame: &[u8]) -> bool {
    if frame.len() < 4 {
        return false;
    }
    let data_len = frame.len() - 2;
    let crc = crc16_modbus(&frame[..data_len]);
    let crc_lo = (crc & 0xFF) as u8;
    let crc_hi = (crc >> 8) as u8;
    crc_lo == frame[data_len] && crc_hi == frame[data_len + 1]
}

/// Append CRC-16 Modbus to a frame buffer at `data_len`.
/// CRC bytes in standard Modbus order (low byte first).
pub fn modbus_append_crc(frame: &mut [u8], data_len: usize) {
    let crc = crc16_modbus(&frame[..data_len]);
    frame[data_len] = (crc & 0xFF) as u8;
    frame[data_len + 1] = (crc >> 8) as u8;
}

/// Build a Modbus read request frame (8 bytes with CRC).
pub fn modbus_build_read_request(slave_addr: u8, func_code: u8, start_reg: u16, count: u16) -> [u8; 8] {
    let mut frame = [0u8; 8];
    frame[0] = slave_addr;
    frame[1] = func_code;
    frame[2] = (start_reg >> 8) as u8;
    frame[3] = (start_reg & 0xFF) as u8;
    frame[4] = (count >> 8) as u8;
    frame[5] = (count & 0xFF) as u8;
    modbus_append_crc(&mut frame, 6);
    frame
}

/// Build a Modbus write single register frame (8 bytes with CRC).
pub fn modbus_build_write_single(slave_addr: u8, register: u16, value: u16) -> [u8; 8] {
    let mut frame = [0u8; 8];
    frame[0] = slave_addr;
    frame[1] = FC_WRITE_SINGLE_REGISTER;
    frame[2] = (register >> 8) as u8;
    frame[3] = (register & 0xFF) as u8;
    frame[4] = (value >> 8) as u8;
    frame[5] = (value & 0xFF) as u8;
    modbus_append_crc(&mut frame, 6);
    frame
}

/// Compute CRC-16 Modbus and return as (high_byte, low_byte) tuple.
/// Compatible with the legacy `calculate_crc16` return format.
pub fn crc16_as_bytes(data: &[u8]) -> (u8, u8) {
    let crc = crc16_modbus(data);
    ((crc >> 8) as u8, (crc & 0xFF) as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modbus_read_request_crc() {
        let frame = modbus_build_read_request(SLAVE_ADDRESS, FC_READ_HOLDING_REGISTERS, 10, 67);
        assert_eq!(frame[0], SLAVE_ADDRESS);
        assert_eq!(frame[1], FC_READ_HOLDING_REGISTERS);
        assert!(modbus_validate_crc(&frame));
    }

    #[test]
    fn test_modbus_write_single_crc() {
        let frame = modbus_build_write_single(SLAVE_ADDRESS, WREG_REQ_BOILER_PWR, 100);
        assert_eq!(frame[0], SLAVE_ADDRESS);
        assert_eq!(frame[1], FC_WRITE_SINGLE_REGISTER);
        assert!(modbus_validate_crc(&frame));
    }

    #[test]
    fn test_modbus_crc_corruption() {
        let mut frame = modbus_build_read_request(0x01, FC_READ_HOLDING_REGISTERS, 0, 10);
        frame[3] ^= 0x01;
        assert!(!modbus_validate_crc(&frame));
    }

    #[test]
    fn test_crc16_as_bytes() {
        let data = [0x50, 0x03, 0x00, 0x0A, 0x00, 0x43];
        let (hi, lo) = crc16_as_bytes(&data);
        let crc = crc16_modbus(&data);
        assert_eq!(hi, (crc >> 8) as u8);
        assert_eq!(lo, (crc & 0xFF) as u8);
    }

    #[test]
    fn test_register_ranges() {
        assert_eq!(READ_REG_COUNT, 67);
        assert_eq!(WRITE_REG_COUNT, 25);
    }
}
