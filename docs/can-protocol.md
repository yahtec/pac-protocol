# CAN Bus Protocol — PAC Hybride

## Overview

CAN bus 500 kbps connects STM32 (master, ID `0x64`) and Screen ESP32 (slave, ID `0x10`).

The protocol uses a register-based polling system where the Screen sequentially requests data from the STM32.

## Frame Format

| Byte | Description |
|------|-------------|
| 0 | Source/Target ID |
| 1 | Function (0x00 = GET, 0x01 = SET) |
| 2 | Register address |
| 3 | Data high byte |
| 4 | Data low byte |
| 5 | CRC-8 (CCITT, polynomial 0x07) |
| 6-7 | Reserved (0x00) |

## Frame IDs

| Constant | Value | Description |
|----------|-------|-------------|
| `CM2_FRAME_ID` | `0x64` (100) | STM32 controller |
| `SCREEN_FRAME_ID` | `0x10` (16) | Screen ESP32 |

## Function Codes

| Constant | Value | Description |
|----------|-------|-------------|
| `CAN_FUNC_GET` | `0x00` | Read register request/response |
| `CAN_FUNC_SET` | `0x01` | Write register |

## Polling Cycle

The Screen polls all 72 registers sequentially, one per CAN frame. At 2ms per register, a full cycle takes ~144ms.

**Priority-based optimization**: SSID frames (cycles 44-50) and version frames (cycles 66-69) are skipped 2 out of 3 full cycles to reduce bus load for non-critical data.

### GET Registers (0x00–0x49)

| Cycle | Addr | Name | Scaled (÷10) |
|-------|------|------|---------------|
| 0 | 0x00 | machine_type | no |
| 1 | 0x01 | bits_states1 | no |
| 2 | 0x02 | bits_states2 | no |
| 3 | 0x03 | bits_states3 | no |
| 4 | 0x04 | temperature_input_boiler | yes |
| 5 | 0x05 | temperature_output_boiler | yes |
| 6 | 0x06 | temperature_smoke_boiler | yes |
| 7 | 0x07 | temperature_input_pac | yes |
| 8 | 0x08 | temperature_bp_pac | yes |
| 9 | 0x09 | temperature_hp_pac | yes |
| 10 | 0x0A | temperature_tank_ecs | yes |
| 11 | 0x0B | temperature_outdoor | yes |
| 12 | 0x0C | temperature_condensation | yes |
| 13 | 0x0D | temperature_evaporation | yes |
| 14 | 0x0E | temperature_overheating | yes |
| 15 | 0x0F | temperature_overcooling | yes |
| 16 | 0x10 | waterflow_boiler | no |
| 17 | 0x11 | waterflow_ecs | no |
| 18 | 0x12 | fan_speed | no |
| 19 | 0x13 | water_pressure | yes |
| 20 | 0x14 | air_pressure | no |
| 21 | 0x15 | pressure_hp | yes |
| 22 | 0x16 | pressure_bp | yes |
| 23 | 0x17 | position_detendeur | no |
| 24-30 | 0x18-0x1E | pump_boiler_* | mixed |
| 31-37 | 0x1F-0x25 | pump_ecs_* | mixed |
| 38 | 0x26 | compressor_status | yes |
| 39-43 | 0x27-0x2B | compressor_output/input_* | mixed |

### SET Registers (0x2C–0x47)

| Cycle | Addr | Name | Description |
|-------|------|------|-------------|
| 44-50 | 0x2C-0x32 | ssid_frames[0-6] | SSID data (7×5 bytes) |
| 51 | 0x33 | general_on | System on/off |
| 52 | 0x34 | boiler_on | Boiler enable |
| 53 | 0x35 | pac_on | PAC enable |
| 54-65 | 0x36-0x41 | Various config | Config registers |
| 66-69 | 0x42-0x45 | version_* | Firmware versions |
| 70-71 | 0x46-0x47 | Reserved | — |

## CRC-8 Calculation

- Polynomial: `0x07` (CRC-8/CCITT)
- Initial value: `0x00`
- Computed over bytes 0-4 of the frame
- Stored in byte 5

## Data Encoding

- All 16-bit values are big-endian (high byte first)
- Scaled registers store `value × 10` on the wire (e.g., 24.5°C → 245)
- Use `data_to_f32_scaled(hi, lo)` to decode scaled values
- Use `f32_to_can_bytes(value, true)` to encode scaled values

## Display Constants

| Constant | Value | Usage |
|----------|-------|-------|
| `FAN_SPEED_DISPLAY_DIVISOR` | 88.0 | Convert raw fan speed to % |
| `STEPPER_DISPLAY_DIVISOR` | 50.0 | Convert raw stepper to % |
| `WATERFLOW_LH_TO_M3H` | 1000.0 | Convert L/h to m³/h |
