/// PAC and Boiler status definitions.
/// Single source of truth for status wire values and display mapping.

// ─── PAC Status Wire Values (sent by STM32 over CAN/SPI) ───

pub const PAC_STATUS_WAITING_FOR_START: u16         = 0;
pub const PAC_STATUS_WATER_FLOW: u16                = 1;
pub const PAC_STATUS_INIT_STEPPER: u16              = 2;
pub const PAC_STATUS_INIT_INVERTER: u16             = 3;
pub const PAC_STATUS_WORKING: u16                   = 4;
pub const PAC_STATUS_FAULT: u16                     = 5;
pub const PAC_STATUS_FAULT_GAS: u16                 = 6;
pub const PAC_STATUS_ANTI_COURT_CYCLE: u16          = 7;
pub const PAC_STATUS_STOPPING: u16                  = 8;
pub const PAC_STATUS_DEFROST: u16                   = 9;
pub const PAC_STATUS_PUMP_DOWN: u16                 = 10;

// ─── Boiler Status Wire Values (sent by STM32 over CAN/SPI) ───

pub const BOILER_STATUS_WAITING_FOR_START: u16           = 0;
pub const BOILER_STATUS_WAIT_FOR_WATER_FLOW: u16         = 1;
pub const BOILER_STATUS_WAIT_FOR_BRAHMA: u16             = 2;
pub const BOILER_STATUS_WAIT_FOR_STARTING_VENTILATION: u16 = 3;
pub const BOILER_STATUS_PREVENTILATION: u16              = 4;
pub const BOILER_STATUS_FLAME_ON: u16                    = 5;
pub const BOILER_STATUS_ATTENTE_REDEMARRAGE_AUTO: u16    = 6;
pub const BOILER_STATUS_STOP_TEMPERATURE_MAX_SD: u16     = 7;
pub const BOILER_STATUS_IN_FAULT: u16                    = 8;
pub const BOILER_STATUS_TRY_NEW_CYCLE: u16               = 9;
pub const BOILER_STATUS_IN_FAULT_GAS: u16                = 10;

// ─── Display Index Mapping ───
// These map wire values to display string array indices for the Screen UI.

/// PAC status display strings (indexed by display index).
/// Use `pac_status_to_display_index()` to convert wire value to display index.
pub const PAC_DISPLAY_STRINGS: &[&str] = &[
    "Arrêt",              // 0
    "Attente débit d'eau", // 1
    "Initialisation",     // 2
    "En fonctionnement",  // 3
    "Arrêt en cours",     // 4
    "Dégivrage",          // 5
    "Anti court-cycle",   // 6
    "Défaut",             // 7
    "Défaut Gaz",         // 8
    "Pump Down",          // 9  ← NEW: was missing, caused PumpDown to show as "Arrêt"
];

/// Boiler status display strings (indexed by display index).
/// Use `boiler_status_to_display_index()` to convert wire value to display index.
pub const BOILER_DISPLAY_STRINGS: &[&str] = &[
    "Arrêt",              // 0
    "Attente débit d'eau", // 1
    "Préventilation",     // 2
    "En fonctionnement",  // 3
    "Arrêt en cours",     // 4
    "Nouvelle tentative", // 5
    "Défaut",             // 6
    "Défaut Gaz",         // 7
    "Arrêt Temp. Max",    // 8  ← NEW: StopTemperatureMaxSD was mapped to generic "Défaut"
];

/// Convert PAC wire status to display index.
/// Returns an index into `PAC_DISPLAY_STRINGS`.
pub fn pac_status_to_display_index(wire_status: u16) -> usize {
    match wire_status {
        PAC_STATUS_WAITING_FOR_START    => 0,  // Arrêt
        PAC_STATUS_WATER_FLOW           => 1,  // Attente débit d'eau
        PAC_STATUS_INIT_STEPPER         => 2,  // Initialisation
        PAC_STATUS_INIT_INVERTER        => 2,  // Initialisation
        PAC_STATUS_WORKING              => 3,  // En fonctionnement
        PAC_STATUS_STOPPING             => 4,  // Arrêt en cours
        PAC_STATUS_DEFROST              => 5,  // Dégivrage
        PAC_STATUS_ANTI_COURT_CYCLE     => 6,  // Anti court-cycle
        PAC_STATUS_FAULT                => 7,  // Défaut
        PAC_STATUS_FAULT_GAS            => 8,  // Défaut Gaz
        PAC_STATUS_PUMP_DOWN            => 9,  // Pump Down
        _                               => 0,  // Default: Arrêt
    }
}

/// Convert Boiler wire status to display index.
/// Returns an index into `BOILER_DISPLAY_STRINGS`.
pub fn boiler_status_to_display_index(wire_status: u16) -> usize {
    match wire_status {
        BOILER_STATUS_WAITING_FOR_START           => 0,  // Arrêt
        BOILER_STATUS_WAIT_FOR_WATER_FLOW         => 1,  // Attente débit d'eau
        BOILER_STATUS_WAIT_FOR_BRAHMA             => 2,  // Préventilation
        BOILER_STATUS_WAIT_FOR_STARTING_VENTILATION => 2,  // Préventilation
        BOILER_STATUS_PREVENTILATION              => 2,  // Préventilation
        BOILER_STATUS_FLAME_ON                    => 3,  // En fonctionnement
        BOILER_STATUS_ATTENTE_REDEMARRAGE_AUTO    => 0,  // Arrêt
        BOILER_STATUS_STOP_TEMPERATURE_MAX_SD     => 8,  // Arrêt Temp. Max (was generic Défaut)
        BOILER_STATUS_IN_FAULT                    => 6,  // Défaut
        BOILER_STATUS_TRY_NEW_CYCLE               => 5,  // Nouvelle tentative
        BOILER_STATUS_IN_FAULT_GAS                => 7,  // Défaut Gaz
        _                                         => 0,  // Default: Arrêt
    }
}
