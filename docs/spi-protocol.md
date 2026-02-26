# SPI Protocol — PAC Hybride

## Overview

SPI at 500 kHz connects STM32 (master) and ESP32 WiFi gateway (slave).
Full-duplex 256-byte frames with CRC-16 Modbus validation.

## Physical Layer

| Parameter | Value |
|-----------|-------|
| Speed | 500 kHz |
| Mode | 0 (CPOL=0, CPHA=0) |
| Frame size | 256 bytes |
| CS management | Software (GPIO PB6 on STM32, GPIO5 on ESP32) |
| Ready signal | ESP32 GPIO2 (high = ready for transfer) |

## Frame Format

```
Byte 0-1:   Message type (big-endian u16)
Byte 2-253: Payload (register data, varies by message type)
Byte 254:   CRC-16 high byte
Byte 255:   CRC-16 low byte
```

**Verify echo**: Bytes 200-201 mirror the message type (used for frame validation).

## CRC-16 Modbus

- Polynomial: `0xA001` (reflected 0x8005)
- Initial value: `0xFFFF`
- Computed over bytes 0-253 (254 bytes)
- Stored **big-endian** at bytes 254-255 (high byte first)

Note: This byte order differs from standard Modbus RTU which uses little-endian CRC.

## Message Types

### Type 1: Output Registers (STM32 → ESP32)

STM32 sends all sensor readings as sequential i16 values starting at byte 2.

```
Byte 0-1: [0x00, 0x01]
Byte 2-3: Register 0 (i16, big-endian)
Byte 4-5: Register 1 (i16, big-endian)
...
Byte 200-201: [0x00, 0x01] (verify)
Byte 254-255: CRC-16
```

Up to 100 registers (200 bytes of data).

### Type 2: Input Registers (STM32 → ESP32)

STM32 sends command/configuration registers written by external Modbus host.

```
Byte 0-1: [0x00, 0x02]
Byte 2-3: protocol_version
Byte 4-5: write_bits_state1
Byte 6-7: write_bits_state2
Byte 8-9: req_boiler_pwr
Byte 10-11: req_temperature_out
Byte 12-13: req_inv_dir
Byte 14-15: req_waterflow
Byte 16-21: reserved
Byte 22-23: t_max_out
Byte 24-25: coef_waterflow
Byte 26-27: p_water_max
Byte 28-29: t_glycol
Byte 30-31: pwr_max
Byte 200-201: [0x00, 0x02] (verify)
Byte 254-255: CRC-16
```

### Type 3: WiFi Data (STM32 → ESP32)

STM32 sends SSID, PAC ID, configuration, fault codes, and version info.

```
Byte 0-1:   [0x00, 0x03] — message type
Byte 2-33:  SSID to connect (32 bytes, raw)
Byte 34-35: PAC ID (big-endian u16)
Byte 36-37: burner_speed_min
Byte 38-39: burner_speed_max
Byte 40-41: burner_speed_prevent
Byte 42-43: oh_regul
Byte 44-45: oh_time_low
Byte 46-47: oh_time_high
Byte 48-49: max_fan_speed
Byte 50-51: step_start_pac_heat
Byte 52-53: step_start_pac_cool
Byte 54-55: fault_pac1
Byte 56-57: fault_pac2
Byte 58-59: fault_gas_pac
Byte 60-61: boiler_fault1
Byte 62-63: temp_max_out
Byte 64-65: coef_waterflow
Byte 66-67: max_water_pressure
Byte 68-69: STM32 version major
Byte 70-71: STM32 version minor
Byte 72-73: Screen version major
Byte 74-75: Screen version minor
Byte 200-201: [0x00, 0x03] (verify)
Byte 254-255: CRC-16
```

### ESP32 → STM32 Responses

#### Control frame (message type 1):

ESP32 sends back Modbus write register values and ESP32 version.

```
Word 0: 0x0000
Word 1: 0x0001 (msg type)
Word 2-8: write registers (protocol_version..req_waterflow)
Word 15: custom_oh (offset by +100)
Word 16: defrost_externe (>=10 means active)
Word 17: ESP32 major version
Word 18: ESP32 minor version
Word 19: combustion_be
Word 20: O2 (×10)
Word 21: NOx (×10)
```

#### WiFi frame (message type 2):

ESP32 sends wifi status, date/time, SSID, and config readback.

```
Word 0: 0x0000
Word 1: 0x0002 (msg type)
Word 2: wifi_connected
Word 3: pac_id
Word 4-8: day, month, year, hour, minute
Word 9-23: SSID connected (15 u16 = 30 bytes)
Word 25: transmit_host_last_10s
Word 26-34: config readback (speeds, overheating, steps)
```

## Cycle Timing

| Side | Cycles | Period |
|------|--------|--------|
| STM32 | 3 (output regs → input regs → wifi) | 200ms between frames |
| ESP32 | 2 (control → wifi) | As fast as master requests |

## Data Encoding

All multi-byte values are **big-endian** (high byte first).

Use pac-protocol helpers:
- `spi_pack_u16(buf, offset, value)` / `spi_unpack_u16(buf, offset)`
- `spi_pack_i16(buf, offset, value)` / `spi_unpack_i16(buf, offset)`
- `spi_append_crc(frame)` / `spi_validate_crc(frame)`
