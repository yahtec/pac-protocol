/// CAN bus protocol definitions for PAC Hybride system.
/// Single source of truth for CAN register IDs, scaling, and frame format.

/// CAN bus bitrate: 500 kbps
pub const CAN_BITRATE: u32 = 500_000;

/// STM32 (CM2) CAN frame ID
pub const CM2_FRAME_ID: u16 = 0x64;

/// Screen CAN frame ID
pub const SCREEN_FRAME_ID: u16 = 0x10;

/// CAN function codes
pub const CAN_FUNC_GET: u8 = 0x00;
pub const CAN_FUNC_SET: u8 = 0x01;

/// Maximum SSID length in bytes
pub const SSID_MAX_LEN: usize = 32;

/// Number of CAN frames used to transmit SSID (7 frames × 5 bytes = 35 bytes, padded)
pub const SSID_CAN_FRAMES: usize = 7;

/// Bytes per SSID CAN frame (bytes 3-7 of the 8-byte CAN frame)
pub const SSID_BYTES_PER_FRAME: usize = 5;

/// Total polling cycle length
pub const CAN_POLL_CYCLE_LENGTH: u8 = 72;

/// CAN register IDs with their properties.
/// Each register has a canonical name, CAN wire ID, and scaling info.
///
/// Naming convention (canonical names across all 3 firmwares):
///   - t1_boiler_in:  Water temperature entering the boiler (= exiting the PAC)
///   - t2_boiler_out: Water temperature leaving the boiler (= départ circuit)
///   - t3_boiler_smoke: Exhaust/smoke temperature
///   - t4_pac_in:     Water temperature entering the PAC (= retour circuit)
///   - t5_bp:         Low-pressure refrigerant temperature
///   - t6_hp:         High-pressure refrigerant temperature (heating side)
///   - t7_hp2:        High-pressure refrigerant temperature (cooling side)
///   - t8_ext:        Exterior ambient temperature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanRegister {
    /// CAN wire ID (byte[2] in the CAN frame)
    pub id: u8,
    /// Polling cycle index (1-72), 0 = SET-only (no polling)
    pub cycle: u8,
    /// Whether the value is scaled ×10 on the wire
    pub scaled: bool,
    /// Whether this is a GET (read) or SET (write) register
    pub is_set: bool,
}

// ─── GET registers (sensor readings from STM32) ───

pub const REG_T1_BOILER_IN: CanRegister          = CanRegister { id: 0x01, cycle: 1,  scaled: true,  is_set: false };
pub const REG_T2_BOILER_OUT: CanRegister         = CanRegister { id: 0x02, cycle: 2,  scaled: true,  is_set: false };
pub const REG_T3_BOILER_SMOKE: CanRegister       = CanRegister { id: 0x03, cycle: 3,  scaled: true,  is_set: false };
pub const REG_T4_PAC_IN: CanRegister             = CanRegister { id: 0x04, cycle: 4,  scaled: true,  is_set: false };
pub const REG_T5_BP: CanRegister                 = CanRegister { id: 0x05, cycle: 5,  scaled: true,  is_set: false };
pub const REG_T6_HP: CanRegister                 = CanRegister { id: 0x06, cycle: 6,  scaled: true,  is_set: false };
pub const REG_T7_HP2: CanRegister                = CanRegister { id: 0x07, cycle: 7,  scaled: true,  is_set: false };
pub const REG_T8_EXT: CanRegister                = CanRegister { id: 0x08, cycle: 8,  scaled: true,  is_set: false };
pub const REG_AIR_PRESSURE: CanRegister          = CanRegister { id: 0x09, cycle: 9,  scaled: true,  is_set: false };
pub const REG_WATER_PRESSURE: CanRegister        = CanRegister { id: 0x0A, cycle: 10, scaled: true,  is_set: false };
pub const REG_P_BP: CanRegister                  = CanRegister { id: 0x0B, cycle: 11, scaled: true,  is_set: false };
pub const REG_P_HP: CanRegister                  = CanRegister { id: 0x0C, cycle: 12, scaled: true,  is_set: false };
pub const REG_POWER_PAC: CanRegister             = CanRegister { id: 0x0D, cycle: 13, scaled: true,  is_set: false };
pub const REG_POWER_BOILER: CanRegister          = CanRegister { id: 0x0E, cycle: 14, scaled: true,  is_set: false };
pub const REG_T_CONDENSATION: CanRegister        = CanRegister { id: 0x0F, cycle: 15, scaled: true,  is_set: false };
pub const REG_T_EVAPORATION: CanRegister         = CanRegister { id: 0x10, cycle: 16, scaled: true,  is_set: false };
pub const REG_OVERHEATING: CanRegister           = CanRegister { id: 0x11, cycle: 17, scaled: true,  is_set: false };
pub const REG_WATERFLOW: CanRegister             = CanRegister { id: 0x12, cycle: 18, scaled: true,  is_set: false };
pub const REG_FAN_SPEED: CanRegister             = CanRegister { id: 0x13, cycle: 19, scaled: true,  is_set: false };
pub const REG_BURNER_SPEED: CanRegister          = CanRegister { id: 0x14, cycle: 20, scaled: false, is_set: false };
pub const REG_T_HP: CanRegister                  = CanRegister { id: 0x15, cycle: 21, scaled: true,  is_set: false };
pub const REG_BRAHMA_FAULT: CanRegister          = CanRegister { id: 0x16, cycle: 22, scaled: true,  is_set: false };
pub const REG_BRAHMA_PWM_FAN_START: CanRegister  = CanRegister { id: 0x17, cycle: 23, scaled: true,  is_set: false };
pub const REG_T_AMBIENT_REGULATION: CanRegister  = CanRegister { id: 0x18, cycle: 24, scaled: true,  is_set: false };
pub const REG_STEPPER_POSITION: CanRegister      = CanRegister { id: 0x19, cycle: 25, scaled: true,  is_set: false };
pub const REG_V4V_MODE: CanRegister              = CanRegister { id: 0x1A, cycle: 26, scaled: true,  is_set: false };
pub const REG_PAC_POWER_PERCENT: CanRegister     = CanRegister { id: 0x1B, cycle: 27, scaled: true,  is_set: false };
pub const REG_ANTI_COURT_CYCLE: CanRegister      = CanRegister { id: 0x1C, cycle: 28, scaled: false, is_set: false };
pub const REG_BOILER_POWER_PERCENT: CanRegister  = CanRegister { id: 0x21, cycle: 33, scaled: true,  is_set: false };
pub const REG_FAULT_PAC1: CanRegister            = CanRegister { id: 0x24, cycle: 36, scaled: false, is_set: false };
pub const REG_FAULT_PAC2: CanRegister            = CanRegister { id: 0x25, cycle: 37, scaled: false, is_set: false };
pub const REG_FAULT_GAS_PAC: CanRegister         = CanRegister { id: 0x26, cycle: 38, scaled: false, is_set: false };
pub const REG_WARN: CanRegister                  = CanRegister { id: 0x27, cycle: 39, scaled: false, is_set: false };
pub const REG_SUBCOOLING: CanRegister            = CanRegister { id: 0x28, cycle: 40, scaled: true,  is_set: false };
pub const REG_FAULT_BOILER: CanRegister          = CanRegister { id: 0x29, cycle: 41, scaled: false, is_set: false };
pub const REG_STATUS_PAC: CanRegister            = CanRegister { id: 0x2A, cycle: 42, scaled: false, is_set: false };
pub const REG_STATUS_BOILER: CanRegister         = CanRegister { id: 0x2B, cycle: 43, scaled: false, is_set: false };
pub const REG_DATE_DAY: CanRegister              = CanRegister { id: 0x36, cycle: 54, scaled: false, is_set: false };
pub const REG_DATE_MONTH: CanRegister            = CanRegister { id: 0x37, cycle: 55, scaled: false, is_set: false };
pub const REG_DATE_YEAR: CanRegister             = CanRegister { id: 0x38, cycle: 56, scaled: false, is_set: false };
pub const REG_TIME_MINUTE: CanRegister           = CanRegister { id: 0x39, cycle: 57, scaled: false, is_set: false };
pub const REG_TIME_HOUR: CanRegister             = CanRegister { id: 0x3A, cycle: 58, scaled: false, is_set: false };
pub const REG_COMBUSTION_BE: CanRegister         = CanRegister { id: 0x3E, cycle: 62, scaled: false, is_set: false };
pub const REG_COMBUSTION_NOX: CanRegister        = CanRegister { id: 0x3F, cycle: 63, scaled: false, is_set: false };
pub const REG_COMBUSTION_O2: CanRegister         = CanRegister { id: 0x40, cycle: 64, scaled: false, is_set: false };
pub const REG_COMBUSTION_MODE: CanRegister       = CanRegister { id: 0x41, cycle: 65, scaled: false, is_set: false };
pub const REG_STM_VERSION_MAJOR: CanRegister     = CanRegister { id: 0x42, cycle: 66, scaled: false, is_set: false };
pub const REG_STM_VERSION_MINOR: CanRegister     = CanRegister { id: 0x43, cycle: 67, scaled: false, is_set: false };
pub const REG_ESP_VERSION_MAJOR: CanRegister     = CanRegister { id: 0x44, cycle: 68, scaled: false, is_set: false };
pub const REG_ESP_VERSION_MINOR: CanRegister     = CanRegister { id: 0x45, cycle: 69, scaled: false, is_set: false };

// ─── SET registers (commands from Screen to STM32) ───

pub const REG_SET_GENERAL: CanRegister           = CanRegister { id: 0x1D, cycle: 29, scaled: false, is_set: true };
pub const REG_SET_PAC: CanRegister               = CanRegister { id: 0x1E, cycle: 30, scaled: false, is_set: true };
pub const REG_SET_BOILER: CanRegister            = CanRegister { id: 0x1F, cycle: 31, scaled: false, is_set: true };
pub const REG_SET_ID: CanRegister                = CanRegister { id: 0x20, cycle: 32, scaled: false, is_set: true };
pub const REG_SET_RESET_PAC: CanRegister         = CanRegister { id: 0x22, cycle: 34, scaled: false, is_set: true };
pub const REG_SET_RESET_BOILER: CanRegister      = CanRegister { id: 0x23, cycle: 35, scaled: false, is_set: true };
pub const REG_SET_SSID_START: u8 = 0x2C; // SSID frames: 0x2C..0x32
pub const REG_SET_WIFI_STATUS: CanRegister       = CanRegister { id: 0x33, cycle: 51, scaled: false, is_set: true };
pub const REG_SET_BOILER_FORCE: CanRegister      = CanRegister { id: 0x34, cycle: 52, scaled: false, is_set: true };
pub const REG_SET_HOST_TRANSMIT: CanRegister     = CanRegister { id: 0x35, cycle: 53, scaled: false, is_set: true };
pub const REG_SET_PUMP_FORCE: CanRegister        = CanRegister { id: 0x3B, cycle: 59, scaled: false, is_set: true };
pub const REG_SET_DEFROST: CanRegister           = CanRegister { id: 0x3C, cycle: 60, scaled: false, is_set: true };
pub const REG_SET_PUMP_DOWN: CanRegister         = CanRegister { id: 0x3D, cycle: 61, scaled: false, is_set: true };
pub const REG_SET_SCREEN_VERSION_MAJOR: CanRegister = CanRegister { id: 0x46, cycle: 70, scaled: false, is_set: true };
pub const REG_SET_SCREEN_VERSION_MINOR: CanRegister = CanRegister { id: 0x47, cycle: 71, scaled: false, is_set: true };
pub const REG_SET_REVERSE_V4V: CanRegister       = CanRegister { id: 0x48, cycle: 72, scaled: false, is_set: true };

/// Display conversion constants for Screen UI.
/// Fan speed: the raw ×10 value needs to be divided by this to get percentage display.
pub const FAN_SPEED_DISPLAY_DIVISOR: f32 = 88.0;

/// Stepper position: the raw ×10 value needs to be divided by this to get percentage display.
pub const STEPPER_DISPLAY_DIVISOR: f32 = 50.0;

/// Waterflow: raw value from CAN is in l/h (×10 scaled on wire).
/// To display in m³/h, divide by 1000 after unscaling.
pub const WATERFLOW_LH_TO_M3H: f32 = 1000.0;
