# HTTP REST API — PAC Hybride

## Overview

The ESP32 WiFi gateway hosts an HTTP server on port 80. A single GET endpoint `/info` returns all system data as JSON and accepts query parameters for setting values.

## Endpoint

```
GET /info
```

### Response

Content-Type: `application/json`
CORS: `Access-Control-Allow-Origin: *`

Returns an `InputRequest` JSON object with all sensor readings, inverter data, pump data, status, configuration, faults, and version information.

### Query Parameters (Write)

Query parameters on the same GET request allow a host application to send control values to the system.

| Parameter | Type | Description |
|-----------|------|-------------|
| `bm` | u16 | Boiler mode |
| `w_bs1` | u16 | Write bit state 1 (also updates host heartbeat) |
| `w_bs2` | u16 | Write bit state 2 |
| `req_boiler_pwr` | u16 | Requested boiler power |
| `req_inv_dir` | f32 | Requested inverter direction |
| `req_t_out` | u16 | Requested output temperature |
| `req_waterflow` | u16 | Requested waterflow |
| `b_spd_min` | u16 | Burner speed minimum |
| `b_spd_max` | u16 | Burner speed maximum |
| `b_spd_prevent` | u16 | Burner speed prevent threshold |
| `oh_rgl` | u16 | Overheating regulation setpoint |
| `oh_tm_l` | u16 | Overheating time low |
| `oh_tm_h` | u16 | Overheating time high |
| `t_max_out` | u16 | Maximum output temperature |
| `coef_waterflow` | u16 | Waterflow coefficient |
| `p_water_max` | u16 | Maximum water pressure |
| `t_glycol` | u16 | Glycol temperature threshold |
| `pwr_max` | u16 | Maximum power |
| `m_dpf_pos` | u16 | Manual DPF position |
| `dt_d`, `dt_m`, `dt_y` | u16 | Date (day, month, year) |
| `tm_m`, `tm_h` | u16 | Time (minute, hour) |
| `max_fan_speed` | u16 | Maximum fan speed percentage |
| `start_step_pac_heat` | u16 | PAC heat start step |
| `start_step_pac_cool` | u16 | PAC cool start step |
| `r_n_com_lost` | — | Reset communication lost counter |
| `be_active` | u16 | Combustion analysis active |
| `o2` | f32 | O2 measurement |
| `nox` | f32 | NOx measurement |

### Example

```
GET /info?w_bs1=1&req_t_out=350&req_boiler_pwr=80
```

## JSON Response Fields

### Sensor Readings

| Field | Type | Scale | Description |
|-------|------|-------|-------------|
| `mt` | u16 | — | Machine type |
| `bs_1`..`bs_3` | u16 | — | Bit states |
| `t1_inter_p_b` | f32 | ÷10 | T input boiler (°C) |
| `t2_out_b` | f32 | ÷10 | T output boiler (°C) |
| `t3_smk_b` | f32 | ÷10 | T smoke boiler (°C) |
| `t4_in_p` | f32 | ÷10 | T input PAC (°C) |
| `t5_bp` | f32 | ÷10 | T BP PAC (°C) |
| `t6_hp_h` | f32 | ÷10 | T HP hot (°C) |
| `t7_hp_c` | f32 | ÷10 | T HP cold (°C) |
| `t8_ext` | f32 | ÷10 | T outdoor (°C) |
| `t_cond` | f32 | ÷10 | T condensation (°C) |
| `t_evap` | f32 | ÷10 | T evaporation (°C) |
| `t_oh` | f32 | ÷10 | T overheating (°C) |
| `t_sc` | f32 | ÷10 | T subcooling (°C) |
| `f_waterflow_b` | f32 | — | Waterflow boiler (L/h) |
| `f_burner_speed` | f32 | — | Burner speed (RPM) |
| `f_fan_speed` | f32 | — | Fan speed |
| `a1_p_water` | f32 | ÷10 | Water pressure (bar) |
| `a2_p_air` | f32 | — | Air pressure |
| `a3_p_hp` | f32 | ÷10 | HP pressure (bar) |
| `a4_p_bp` | f32 | ÷10 | BP pressure (bar) |
| `dpf_pos` | i16 | — | Expansion valve position |

### Boiler Pump (Grundfos Para Maxo)

| Field | Type | Description |
|-------|------|-------------|
| `b_p_dp` | f32 | Differential pressure |
| `b_p_waterflow` | f32 | Waterflow |
| `b_p_energy` | f32 | Energy |
| `b_p_power` | f32 | Power |
| `b_p_time` | f32 | Operating time |
| `b_p_speed` | f32 | Speed |
| `b_p_mode` | f32 | Operating mode |

### Inverter (Sanhua SD2)

| Field | Type | Description |
|-------|------|-------------|
| `inv_s` | f32 | Status |
| `inv_out_freq` | f32 | Output frequency |
| `inv_out_volt` | f32 | Output voltage |
| `inv_out_cur` | f32 | Output current |
| `inv_out_pwr` | f32 | Output power |
| `inv_in_volt` | f32 | Input voltage |
| `inv_in_cur` | f32 | Input current |
| `inv_in_pwr` | f32 | Input power |
| `inv_t_ipm` | f32 | IPM temperature |
| `inv_t_pfc` | f32 | PFC module temperature |
| `inv_t_pcb` | f32 | PCB temperature |
| `inv_fault`..`inv_fault2` | f32 | Fault codes |

### Status

| Field | Type | Description |
|-------|------|-------------|
| `s_boiler` | u16 | Boiler status code |
| `s_pac` | u16 | PAC status code |
| `s_available` | u16 | Source available |
| `fan_speed_applied` | f32 | Applied fan speed |
| `inv_freq_applied` | f32 | Applied inverter frequency |
| `acc` | u16 | Anti-short-cycle counter |

### Versions & Diagnostics

| Field | Type | Description |
|-------|------|-------------|
| `v_esp` | string | ESP32 firmware version (e.g., "3.2") |
| `v_stm` | string | STM32 firmware version |
| `v_screen` | string | Screen firmware version |
| `esp_time` | u64 | Request counter (increments per /info call) |
| `n_com_lost` | u16 | Communication lost counter |
| `n_com_wifi_lost` | u16 | WiFi communication lost counter |

## Connection Monitoring

The host should send `w_bs1=1` regularly. If no request with `w_bs1` is received for 120 seconds, the ESP32 considers the connection lost and increments `n_com_lost`.
