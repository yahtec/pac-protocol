/// HTTP REST API definitions for PAC Hybride system.
///
/// The ESP32 WiFi gateway exposes a single GET endpoint `/info` that returns
/// all system data as JSON. Query parameters allow setting output values.
///
/// This module requires the `std` feature for String and serde support.

use serde::{Deserialize, Serialize};

/// JSON response struct for the `/info` HTTP endpoint.
/// Contains all sensor readings, inverter data, pump data, status,
/// configuration, and version information.
#[derive(Serialize, Deserialize)]
pub struct InputRequest {
    // Sensor readings (temperatures scaled ÷10 where noted)
    pub mt: u16,
    pub bs_1: u16,
    pub bs_2: u16,
    pub bs_3: u16,
    pub t1_inter_p_b: f32,
    pub t2_out_b: f32,
    pub t3_smk_b: f32,
    pub t4_in_p: f32,
    pub t5_bp: f32,
    pub t6_hp_h: f32,
    pub t7_hp_c: f32,
    pub t8_ext: f32,
    pub t_cond: f32,
    pub t_evap: f32,
    pub t_oh: f32,
    pub t_sc: f32,
    pub f_waterflow_b: f32,
    pub f_waterflow_ecs: f32,
    pub f_burner_speed: f32,
    pub f_fan_speed: f32,
    pub a1_p_water: f32,
    pub a2_p_air: f32,
    pub a3_p_hp: f32,
    pub a4_p_bp: f32,
    pub dpf_pos: i16,
    // Boiler pump (Grundfos Para Maxo)
    pub b_p_dp: f32,
    pub b_p_waterflow: f32,
    pub b_p_energy: f32,
    pub b_p_power: f32,
    pub b_p_time: f32,
    pub b_p_speed: f32,
    pub b_p_mode: f32,
    // Inverter (Sanhua SD2)
    pub inv_s: f32,
    pub inv_out_freq: f32,
    pub inv_out_volt: f32,
    pub inv_out_cur: f32,
    pub inv_out_pwr: f32,
    pub inv_in_volt: f32,
    pub inv_in_cur: f32,
    pub inv_in_pwr: f32,
    pub inv_t_ipm: f32,
    pub inv_t_pfc: f32,
    pub inv_t_pcb: f32,
    pub inv_fault: f32,
    pub inv_fault1: f32,
    pub inv_fault2: f32,
    // Status
    pub s_boiler: u16,
    pub s_pac: u16,
    pub s_available: u16,
    pub fan_speed_applied: f32,
    pub inv_freq_applied: f32,
    pub acc: u16,
    // Output/command data
    pub bm: u16,
    pub w_bs1: u16,
    pub w_bs2: u16,
    pub req_boiler_pwr: u16,
    pub req_inv_dir: f32,
    pub req_t_out: u16,
    pub req_wf: u16,
    pub dt_d: u16,
    pub dt_m: u16,
    pub dt_y: u16,
    pub tm_m: u16,
    pub tm_h: u16,
    // Configuration
    pub b_spd_min: u16,
    pub b_spd_max: u16,
    pub b_spd_prevent: u16,
    pub oh_rgl: u16,
    pub oh_tm_l: u16,
    pub oh_tm_h: u16,
    pub t_max_out: u16,
    pub coef_wf: u16,
    pub p_water_max: u16,
    pub t_glycol: u16,
    pub pwr_max: u16,
    pub m_dpf_pos: u16,
    pub max_fan_speed: u16,
    pub start_step_pac_heat: u16,
    pub start_step_pac_cool: u16,
    pub esp_time: u64,
    // Faults
    pub pac_fault1: u16,
    pub pac_fault2: u16,
    pub pac_fault3: u16,
    pub boiler_fault1: u16,
    // Versions
    pub v_esp: String,
    pub v_stm: String,
    pub v_screen: String,
    // Diagnostics
    pub n_com_lost: u16,
    pub n_com_wifi_lost: u16,
    pub reverse_v4v: u16,
    // Combustion
    pub be_active: u16,
    pub o2: f32,
    pub nox: f32,
}

/// HTTP query parameter names for the `/info` endpoint.
/// These are the keys accepted in the query string to set output values.
pub const HTTP_PARAMS: &[&str] = &[
    "bm", "w_bs1", "w_bs2", "req_boiler_pwr", "req_inv_dir", "req_t_out",
    "req_waterflow", "b_spd_min", "b_spd_max", "b_spd_prevent",
    "oh_rgl", "oh_tm_l", "oh_tm_h", "t_max_out", "coef_waterflow",
    "p_water_max", "t_glycol", "pwr_max", "m_dpf_pos",
    "dt_d", "dt_m", "dt_y", "tm_m", "tm_h",
    "max_fan_speed", "start_step_pac_heat", "start_step_pac_cool",
    "r_n_com_lost", "be_active", "o2", "nox",
];
