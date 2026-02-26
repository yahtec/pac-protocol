# Modbus RTU Protocol — PAC Hybride

## Overview

The STM32 controller acts as both a Modbus slave and master on UART interfaces.

- **Slave** (address `0x50`): Exposes read registers 10-76 and write registers 100-124 to an external host
- **Master**: Queries the inverter (Sanhua SD2), boiler pump (Grundfos Para Maxo), and gas sensors via separate UART interfaces

## Physical Layer

| Parameter | Value |
|-----------|-------|
| Baud rate | 9600 (slave), varies by device (master) |
| Data bits | 8 |
| Parity | None (slave), varies by device (master) |
| Stop bits | 1 |

## CRC-16 Modbus

- Polynomial: `0xA001` (reflected 0x8005)
- Initial value: `0xFFFF`
- Byte order: **Little-endian** (low byte first, standard Modbus RTU)

## Function Codes

| Code | Constant | Description |
|------|----------|-------------|
| `0x03` | `FC_READ_HOLDING_REGISTERS` | Read holding registers |
| `0x04` | `FC_READ_INPUT_REGISTERS` | Read input registers |
| `0x06` | `FC_WRITE_SINGLE_REGISTER` | Write single register |
| `0x10` | `FC_WRITE_MULTIPLE_REGISTERS` | Write multiple registers |

## Slave Register Map

### Read Registers (Address 10-76)

These expose all sensor data. An external Modbus host reads these to monitor the system.

| Address | Constant | Description | Scale |
|---------|----------|-------------|-------|
| 10 | `REG_MACHINE_TYPE` | Machine type | — |
| 11 | `REG_BITS_STATES1` | Bit states word 1 | — |
| 12 | `REG_BITS_STATES2` | Bit states word 2 | — |
| 13 | `REG_BITS_STATES3` | Bit states word 3 | — |
| 14 | `REG_T_INPUT_BOILER` | T input boiler (°C) | ×10 |
| 15 | `REG_T_OUTPUT_BOILER` | T output boiler (°C) | ×10 |
| 16 | `REG_T_SMOKE_BOILER` | T smoke boiler (°C) | ×10 |
| 17 | `REG_T_INPUT_PAC` | T input PAC (°C) | ×10 |
| 18 | `REG_T_BP_PAC` | T BP PAC (°C) | ×10 |
| 19 | `REG_T_HP_PAC` | T HP PAC (°C) | ×10 |
| 20 | `REG_T_TANK_ECS` | T tank ECS (°C) | ×10 |
| 21 | `REG_T_OUTDOOR` | T outdoor (°C) | ×10 |
| 22 | `REG_T_CONDENSATION` | T condensation (°C) | ×10 |
| 23 | `REG_T_EVAPORATION` | T evaporation (°C) | ×10 |
| 24 | `REG_T_OVERHEATING` | T overheating (°C) | ×10 |
| 25 | `REG_T_OVERCOOLING` | T overcooling (°C) | ×10 |
| 26 | `REG_WATERFLOW_BOILER` | Waterflow boiler | — |
| 27 | `REG_WATERFLOW_ECS` | Waterflow ECS | — |
| 28 | `REG_FAN_SPEED` | Burner fan speed | — |
| 29 | `REG_WATER_PRESSURE` | Water pressure | ×10 |
| 30 | `REG_AIR_PRESSURE` | Air pressure | — |
| 31 | `REG_FAN_AIR_PRESSURE` | Fan air pressure | — |
| 32 | `REG_PRESSURE_HP` | HP pressure (bar) | ×10 |
| 33 | `REG_PRESSURE_BP` | BP pressure (bar) | ×10 |
| 34 | `REG_POSITION_DETENDEUR` | Expansion valve position | — |
| 35-41 | `REG_PUMP_BOILER_*` | Boiler pump (DP, WF, Energy, Power, Time, Speed, Mode) | mixed |
| 42-48 | `REG_PUMP_ECS_*` | ECS pump (same fields) | mixed |
| 49-55 | `REG_PUMP_LOOP_*` | Loop pump (same fields) | mixed |
| 56 | `REG_COMPRESSOR_STATUS` | Compressor status | ×10 |
| 57 | `REG_COMPRESSOR_OUT_FREQ` | Output frequency (Hz) | ×10 |
| 58 | `REG_COMPRESSOR_OUT_VOLT` | Output voltage (V) | ×10 |
| 59 | `REG_COMPRESSOR_OUT_CURRENT` | Output current (A) | ×10 |
| 60 | `REG_COMPRESSOR_OUT_POWER` | Output power (W) | — |
| 61 | `REG_COMPRESSOR_IN_VOLT` | Input voltage (V) | ×10 |
| 62 | `REG_COMPRESSOR_IN_CURRENT` | Input current (A) | ×10 |
| 63 | `REG_COMPRESSOR_IN_POWER` | Input power (W) | — |
| 64 | `REG_INVERTER_IPM_TEMP` | IPM temperature (°C) | ×10 |
| 65 | `REG_PFC_MODULE_TEMP` | PFC module temperature (°C) | ×10 |
| 66 | `REG_PCP_TEMP` | PCB temperature (°C) | ×10 |
| 67-69 | `REG_COMPRESSOR_FAULT*` | Fault codes 0-2 | ×10 |
| 70 | `REG_FAULT_CURRENT` | Current fault code | — |
| 71 | `REG_STATUS_BOILER` | Boiler status | — |
| 72 | `REG_STATUS_PAC` | PAC status | — |
| 73 | `REG_SOURCE_AVAILABLE` | Source available | — |
| 74 | `REG_FAN_SPEED_PAC` | PAC fan speed | — |
| 75 | `REG_COMPRESSOR_IN_FREQ` | Applied input frequency | ×10 |
| 76 | `REG_ANTI_SHORT_CYCLE` | Anti-short-cycle counter | — |

### Write Registers (Address 100-124)

External host writes these to control the system.

| Address | Constant | Description |
|---------|----------|-------------|
| 100 | `WREG_PROTOCOL_VERSION` | Protocol version |
| 101 | `WREG_BITS_STATE1` | Control bit state 1 |
| 102 | `WREG_BITS_STATE2` | Control bit state 2 |
| 103 | `WREG_REQ_BOILER_PWR` | Requested boiler power |
| 104 | `WREG_REQ_TEMPERATURE_OUT` | Requested output temperature |
| 105 | `WREG_REQ_INV_DIR` | Requested inverter direction |
| 106 | `WREG_REQ_WATERFLOW` | Requested waterflow |
| 107 | `WREG_BOILER_SPEED_MIN` | Burner speed minimum |
| 108 | `WREG_BOILER_SPEED_MAX` | Burner speed maximum |
| 109 | `WREG_BOILER_SPEED_PREVENT` | Burner speed prevent |
| 110 | `WREG_FAN_SPEED_PAC` | PAC fan speed |
| 111 | `WREG_COMPRESSOR_FREQ` | Compressor frequency |
| 112 | `WREG_EXPANSION_VALVE_POS` | Expansion valve position |
| 113 | `WREG_PUMP_FORCE_SPEED` | Pump force speed |
| 114 | `WREG_PUMP_ECS_FORCE_SPEED` | ECS pump force speed |
| 115 | `WREG_T_MAX_OUT` | Max output temperature |
| 116-118 | `WREG_P`, `WREG_I`, `WREG_D` | PID parameters |
| 119 | `WREG_COEF_WATERFLOW` | Waterflow coefficient |
| 120 | `WREG_P_WATER_MAX` | Max water pressure |
| 121 | `WREG_T_GLYCOL` | Glycol temperature |
| 122 | `WREG_PWR_MAX` | Maximum power |
| 123 | `WREG_O2` | O2 measurement |
| 124 | `WREG_NOX` | NOx measurement |

## Master Devices

### Inverter — Sanhua SD2

Variable-frequency drive for the compressor.

| Register | Direction | Description |
|----------|-----------|-------------|
| 0-28 | Read | Status, frequency, voltage, current, power, temperatures, faults |
| 0 | Write | Target frequency |
| 3 | Write | Rotation direction |

### Boiler Pump — Grundfos Para Maxo

Circulation pump with Modbus interface.

| Register | Direction | Description |
|----------|-----------|-------------|
| 1-7 | Read | Differential pressure, waterflow, energy, power, operating time, speed, mode |
| 1 | Write | Setpoint |
| 40 | Write | Control mode |
| 42 | Write | Stop/Start |

### Gas Sensors (R290 / G20)

Refrigerant and gas leak detection sensors.

| Register | Direction | Description |
|----------|-----------|-------------|
| 0x110-0x115 | Read | Gas concentration and status |
| 0x101 | Write | Zero calibration |

## Frame Helpers

pac-protocol provides ready-to-use frame builders:

```rust
// Build a read request
let frame = modbus_build_read_request(SLAVE_ADDRESS, FC_READ_HOLDING_REGISTERS, 10, 67);

// Build a write single register
let frame = modbus_build_write_single(SLAVE_ADDRESS, WREG_REQ_BOILER_PWR, 100);

// Validate received frame
if modbus_validate_crc(&received_frame) { ... }
```
