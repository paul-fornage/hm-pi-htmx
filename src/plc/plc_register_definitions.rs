
use crate::modbus::{ModbusAddressType, RegisterAddress, RegisterMetadata};
use crate::modbus::cached_modbus::ModbusChunk;

pub const CLEARCORE_CHUNKS: &'static[ModbusChunk] = &[
    ModbusChunk::Coils{address: 0, count: 68},
    ModbusChunk::DiscreteInputs{address: 0, count: 1},
    ModbusChunk::InputRegisters{address: 0, count: 2},
    ModbusChunk::HoldingRegisters{address: 0, count: 86},
];

// ============================================================================
// COILS
// ============================================================================

pub const CONFIG_READY: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 1 },
    name: "CONFIG READY",
    description: "HMI sets to true after uploading all options",
};

pub const IS_OP_CANCELLABLE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 9 },
    name: "IS OP CANCELLABLE",
    description: "Is the current operation cancellable (should display cancel button)",
};

pub const START_CYCLE_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 10 },
    name: "START CYCLE LATCH",
    description: "Start cycle latch",
};

pub const HOME_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 11 },
    name: "HOME LATCH",
    description: "Home latch",
};

pub const IS_HOMED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 12 },
    name: "IS HOMED",
    description: "Is homed",
};

pub const AT_START: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 13 },
    name: "AT START",
    description: "At start position",
};

pub const GO_TO_START_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 14 },
    name: "GO TO START LATCH",
    description: "Go to start latch",
};

pub const JOB_ACTIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 15 },
    name: "JOB ACTIVE",
    description: "Job is active",
};

pub const CANCEL_OPERATION_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 16 },
    name: "CANCEL OPERATION LATCH",
    description: "Cancel operation latch",
};

pub const FORCE_ESTOP: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 17 },
    name: "FORCE ESTOP",
    description: "HMI commands estop",
};

pub const WELD_ENABLE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 18 },
    name: "WELD ENABLE",
    description: "If false, it will be in sim mode",
};

pub const WELD_SIGNAL: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 19 },
    name: "WELD SIGNAL",
    description: "Weld signal",
};

pub const IN_ESTOP: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 20 },
    name: "IN ESTOP",
    description: "Currently in emergency stop",
};

pub const ERROR: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 21 },
    name: "ERROR",
    description: "Error state",
};

pub const COMMANDED_FF_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 32 },
    name: "COMMANDED FF LATCH",
    description: "Commanded fast forward latch",
};

pub const COMMANDED_RF_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 33 },
    name: "COMMANDED RF LATCH",
    description: "Commanded rewind/reverse latch",
};

pub const JOG_X_AXIS_POSITIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 40 },
    name: "JOG X AXIS POSITIVE",
    description: "Jog X axis in positive direction",
};

pub const JOG_X_AXIS_NEGATIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 41 },
    name: "JOG X AXIS NEGATIVE",
    description: "Jog X axis in negative direction",
};

pub const JOG_Y_AXIS_POSITIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 42 },
    name: "JOG Y AXIS POSITIVE",
    description: "Jog Y axis in positive direction",
};

pub const JOG_Y_AXIS_NEGATIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 43 },
    name: "JOG Y AXIS NEGATIVE",
    description: "Jog Y axis in negative direction",
};

pub const JOG_Z_AXIS_POSITIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 44 },
    name: "JOG Z AXIS POSITIVE",
    description: "Jog Z axis in positive direction",
};

pub const JOG_Z_AXIS_NEGATIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 45 },
    name: "JOG Z AXIS NEGATIVE",
    description: "Jog Z axis in negative direction",
};

pub const JOG_W_AXIS_POSITIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 46 },
    name: "JOG W AXIS POSITIVE",
    description: "Jog W axis in positive direction",
};

pub const JOG_W_AXIS_NEGATIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 47 },
    name: "JOG W AXIS NEGATIVE",
    description: "Jog W axis in negative direction",
};

pub const AXIS_X_HOME_DIRECTION_POSITIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 50 },
    name: "AXIS X HOME DIRECTION",
    description: "Direction to home the X axis motor. True is positive",
};

pub const AXIS_Y_HOME_DIRECTION_POSITIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 51 },
    name: "AXIS Y HOME DIRECTION",
    description: "Direction to home the Y axis motor. True is positive",
};

pub const AXIS_Z_HOME_DIRECTION_POSITIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 52 },
    name: "AXIS Z HOME DIRECTION",
    description: "Direction to home the Z axis motor. True is positive",
};

pub const USES_Y_AXIS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 65 },
    name: "USES Y AXIS",
    description: "Machine uses Y axis",
};

pub const USES_Z_AXIS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 66 },
    name: "USES Z AXIS",
    description: "Machine uses Z axis",
};

pub const USES_W_AXIS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 67 },
    name: "USES W AXIS",
    description: "Machine uses W axis",
};

// ============================================================================
// DISCRETE INPUTS
// ============================================================================

pub const FORGOR: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 0 },
    name: "FORGOR",
    description: "Discrete input (TODO: define purpose)",
};

// ============================================================================
// INPUT REGISTERS
// ============================================================================

pub const SERIAL_NUMBER_LOW: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 0 },
    name: "SERIAL NUMBER LOW",
    description: "Serial number low word",
};

pub const SERIAL_NUMBER_HIGH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 1 },
    name: "SERIAL NUMBER HIGH",
    description: "Serial number high word",
};

// ============================================================================
// HOLDING REGISTERS
// ============================================================================

pub const CURRENT_POSITION: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 8 },
    name: "CURRENT POSITION",
    description: "Current position of carriage (hundredths of an inch)",
};

pub const CYCLE_START_POS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 13 },
    name: "CYCLE START POS",
    description: "Distance from 0 to start weld (hundredths of an inch)",
};

pub const CYCLE_END_POS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 14 },
    name: "CYCLE END POS",
    description: "Distance from 0 to finish weld (hundredths of an inch)",
};

pub const CYCLE_PARK_POS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 15 },
    name: "CYCLE PARK POS",
    description: "Distance from 0 to park after weld cycle (hundredths of an inch)",
};

pub const CYCLE_WELD_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 16 },
    name: "CYCLE WELD SPEED",
    description: "Speed to weld (hundredths of an inch per minute)",
};

pub const CYCLE_REPOSITION_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 17 },
    name: "CYCLE REPOSITION SPEED",
    description: "Speed to move carriage when not actively welding (hundredths of an inch per minute)",
};

pub const CYCLE_WIRE_FEED_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 18 },
    name: "CYCLE WIRE FEED SPEED",
    description: "Speed to extrude wire while welding (hundredths of an inch per minute)",
};

pub const AXIS_X_COMMANDED_JOG_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 20 },
    name: "AXIS X COMMANDED JOG SPEED",
    description: "Currently commanded jogging speed for X axis (hundredths of an inch per minute)",
};

pub const AXIS_Y_COMMANDED_JOG_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 21 },
    name: "AXIS Y COMMANDED JOG SPEED",
    description: "Currently commanded jogging speed for Y axis (hundredths of an inch per minute)",
};

pub const AXIS_Z_COMMANDED_JOG_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 22 },
    name: "AXIS Z COMMANDED JOG SPEED",
    description: "Currently commanded jogging speed for Z axis (hundredths of an inch per minute)",
};

pub const AXIS_W_COMMANDED_JOG_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 23 },
    name: "AXIS W COMMANDED JOG SPEED",
    description: "Currently commanded jogging speed for W axis (hundredths of an inch per minute)",
};

pub const AXIS_X_HOMING_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 24 },
    name: "AXIS X HOMING SPEED",
    description: "Speed to use when homing X axis (hundredths of an inch per minute)",
};

pub const AXIS_Y_HOMING_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 25 },
    name: "AXIS Y HOMING SPEED",
    description: "Speed to use when homing Y axis (hundredths of an inch per minute)",
};

pub const AXIS_Z_HOMING_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 26 },
    name: "AXIS Z HOMING SPEED",
    description: "Speed to use when homing Z axis (hundredths of an inch per minute)",
};

pub const AXIS_X_HOMING_OFFSET: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 27 },
    name: "AXIS X HOMING OFFSET",
    description: "Distance to move X axis away from hardware limit after homing (hundredths of an inch)",
};

pub const AXIS_Y_HOMING_OFFSET: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 28 },
    name: "AXIS Y HOMING OFFSET",
    description: "Distance to move Y axis away from hardware limit after homing (hundredths of an inch)",
};

pub const AXIS_Z_HOMING_OFFSET: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 29 },
    name: "AXIS Z HOMING OFFSET",
    description: "Distance to move Z axis away from hardware limit after homing (hundredths of an inch)",
};

pub const CYCLE_PROGRESS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 30 },
    name: "CYCLE PROGRESS",
    description: "Progress on current job (hundredths of percent, 0..10000)",
};

pub const INCHES_PER_STEP_X_AXIS_LOWER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 64 },
    name: "X AXIS SCALING",
    description: "u32 where combined value over 10^9 is inches per step. range: 0..4.294967296",
};

pub const INCHES_PER_STEP_X_AXIS_UPPER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 65 },
    name: "X AXIS SCALING UPPER",
    description: "See HUNDREDTHS_PER_STEP_X_AXIS_LOWER",
};

pub const INCHES_PER_STEP_Y_AXIS_LOWER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 66 },
    name: "Y AXIS SCALING",
    description: "u32 where combined value over 10^9 is inches per step. range: 0..4.294967296",
};

pub const INCHES_PER_STEP_Y_AXIS_UPPER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 67 },
    name: "Y AXIS SCALING UPPER",
    description: "See HUNDREDTHS_PER_STEP_Y_AXIS_LOWER",
};

pub const INCHES_PER_STEP_Z_AXIS_LOWER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 68 },
    name: "Z AXIS SCALING",
    description: "u32 where combined value over 10^9 is inches per step. range: 0..4.294967296",
};

pub const INCHES_PER_STEP_Z_AXIS_UPPER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 69 },
    name: "Z AXIS SCALING UPPER",
    description: "See HUNDREDTHS_PER_STEP_Z_AXIS_LOWER",
};

pub const INCHES_PER_STEP_W_AXIS_LOWER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 70 },
    name: "W AXIS SCALING",
    description: "u32 where combined value over 10^9 is inches per step. range: 0..4.294967296",
};

pub const INCHES_PER_STEP_W_AXIS_UPPER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 71 },
    name: "W AXIS SCALING UPPER",
    description: "See HUNDREDTHS_PER_STEP_W_AXIS_LOWER",
};

pub const MIN_POS_X_AXIS_HUNDREDTHS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 72 },
    name: "MIN POS X AXIS",
    description: "Minimum position for X axis (hundredths of an inch)",
};

pub const MAX_POS_X_AXIS_HUNDREDTHS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 73 },
    name: "MAX POS X AXIS",
    description: "Maximum position for X axis (hundredths of an inch)",
};

pub const MAX_VEL_X_AXIS_HUNDREDTHS_PER_MINUTE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 74 },
    name: "MAX VEL X AXIS",
    description: "Maximum velocity for X axis (hundredths of an inch per minute)",
};

pub const MAX_ACCEL_X_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 75 },
    name: "MAX ACCEL X AXIS",
    description: "Maximum acceleration for X axis (hundredths of an inch per minute per second)",
};

pub const MIN_POS_Y_AXIS_HUNDREDTHS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 76 },
    name: "MIN POS Y AXIS",
    description: "Minimum position for Y axis (hundredths of an inch)",
};

pub const MAX_POS_Y_AXIS_HUNDREDTHS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 77 },
    name: "MAX POS Y AXIS",
    description: "Maximum position for Y axis (hundredths of an inch)",
};

pub const MAX_VEL_Y_AXIS_HUNDREDTHS_PER_MINUTE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 78 },
    name: "MAX VEL Y AXIS",
    description: "Maximum velocity for Y axis (hundredths of an inch per minute)",
};

pub const MAX_ACCEL_Y_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 79 },
    name: "MAX ACCEL Y AXIS",
    description: "Maximum acceleration for Y axis (hundredths of an inch per minute per second)",
};

pub const MIN_POS_Z_AXIS_HUNDREDTHS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 80 },
    name: "MIN POS Z AXIS",
    description: "Minimum position for Z axis (hundredths of an inch)",
};

pub const MAX_POS_Z_AXIS_HUNDREDTHS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 81 },
    name: "MAX POS Z AXIS",
    description: "Maximum position for Z axis (hundredths of an inch)",
};

pub const MAX_VEL_Z_AXIS_HUNDREDTHS_PER_MINUTE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 82 },
    name: "MAX VEL Z AXIS",
    description: "Maximum velocity for Z axis (hundredths of an inch per minute)",
};

pub const MAX_ACCEL_Z_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 83 },
    name: "MAX ACCEL Z AXIS",
    description: "Maximum acceleration for Z axis (hundredths of an inch per minute per second)",
};

pub const MAX_VEL_W_AXIS_HUNDREDTHS_PER_MINUTE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 84 },
    name: "MAX VEL W AXIS",
    description: "Maximum velocity for W axis (hundredths of an inch per minute)",
};

pub const MAX_ACCEL_W_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 85 },
    name: "MAX ACCEL W AXIS",
    description: "Maximum acceleration for W axis (hundredths of an inch per minute per second)",
};



pub const CLEARCORE_REGISTERS: &'static[RegisterMetadata] = &[
    CONFIG_READY,
    IS_OP_CANCELLABLE,
    START_CYCLE_LATCH,
    HOME_LATCH,
    IS_HOMED,
    AT_START,
    GO_TO_START_LATCH,
    JOB_ACTIVE,
    CANCEL_OPERATION_LATCH,
    FORCE_ESTOP,
    WELD_ENABLE,
    WELD_SIGNAL,
    IN_ESTOP,
    ERROR,
    COMMANDED_FF_LATCH,
    COMMANDED_RF_LATCH,
    JOG_X_AXIS_POSITIVE,
    JOG_X_AXIS_NEGATIVE,
    JOG_Y_AXIS_POSITIVE,
    JOG_Y_AXIS_NEGATIVE,
    JOG_Z_AXIS_POSITIVE,
    JOG_Z_AXIS_NEGATIVE,
    JOG_W_AXIS_POSITIVE,
    JOG_W_AXIS_NEGATIVE,
    AXIS_X_HOME_DIRECTION_POSITIVE,
    AXIS_Y_HOME_DIRECTION_POSITIVE,
    AXIS_Z_HOME_DIRECTION_POSITIVE,
    USES_Y_AXIS,
    USES_Z_AXIS,
    USES_W_AXIS,
    FORGOR,
    SERIAL_NUMBER_LOW,
    SERIAL_NUMBER_HIGH,
    CURRENT_POSITION,
    CYCLE_START_POS,
    CYCLE_END_POS,
    CYCLE_PARK_POS,
    CYCLE_WELD_SPEED,
    CYCLE_REPOSITION_SPEED,
    CYCLE_WIRE_FEED_SPEED,
    AXIS_X_COMMANDED_JOG_SPEED,
    AXIS_Y_COMMANDED_JOG_SPEED,
    AXIS_Z_COMMANDED_JOG_SPEED,
    AXIS_W_COMMANDED_JOG_SPEED,
    AXIS_X_HOMING_SPEED,
    AXIS_Y_HOMING_SPEED,
    AXIS_Z_HOMING_SPEED,
    AXIS_X_HOMING_OFFSET,
    AXIS_Y_HOMING_OFFSET,
    AXIS_Z_HOMING_OFFSET,
    CYCLE_PROGRESS,
    INCHES_PER_STEP_X_AXIS_LOWER,
    INCHES_PER_STEP_X_AXIS_UPPER,
    INCHES_PER_STEP_Y_AXIS_LOWER,
    INCHES_PER_STEP_Y_AXIS_UPPER,
    INCHES_PER_STEP_Z_AXIS_LOWER,
    INCHES_PER_STEP_Z_AXIS_UPPER,
    INCHES_PER_STEP_W_AXIS_LOWER,
    INCHES_PER_STEP_W_AXIS_UPPER,
    MIN_POS_X_AXIS_HUNDREDTHS,
    MAX_POS_X_AXIS_HUNDREDTHS,
    MAX_VEL_X_AXIS_HUNDREDTHS_PER_MINUTE,
    MAX_ACCEL_X_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND,
    MIN_POS_Y_AXIS_HUNDREDTHS,
    MAX_POS_Y_AXIS_HUNDREDTHS,
    MAX_VEL_Y_AXIS_HUNDREDTHS_PER_MINUTE,
    MAX_ACCEL_Y_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND,
    MIN_POS_Z_AXIS_HUNDREDTHS,
    MAX_POS_Z_AXIS_HUNDREDTHS,
    MAX_VEL_Z_AXIS_HUNDREDTHS_PER_MINUTE,
    MAX_ACCEL_Z_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND,
    MAX_VEL_W_AXIS_HUNDREDTHS_PER_MINUTE,
    MAX_ACCEL_W_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND,
];


pub fn get_clearcore_register_metadata(register_name: &str) -> Option<&'static RegisterMetadata> {
    static REGISTER_MAP: std::sync::OnceLock<std::collections::HashMap<&'static str, &'static RegisterMetadata>> = std::sync::OnceLock::new();

    let map = REGISTER_MAP.get_or_init(|| {
        CLEARCORE_REGISTERS.iter().map(|r| (r.name, r)).collect()
    });

    map.get(register_name).copied()
}