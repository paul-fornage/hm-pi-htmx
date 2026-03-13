use crate::modbus::{ModbusAddressType, RegisterAddress, RegisterMetadata};
use crate::modbus::cached_modbus::ModbusChunk;

pub const CLEARCORE_CHUNKS: &'static[ModbusChunk] = &[
    ModbusChunk::Coils{address: 0, count: 128},
    ModbusChunk::DiscreteInputs{address: 0, count: 128},
    ModbusChunk::InputRegisters{address: 0, count: 64},
    ModbusChunk::HoldingRegisters{address: 0, count: 64},
    ModbusChunk::HoldingRegisters{address: 64, count: 64},
];

// ============================================================================
// COILS
// ============================================================================

/// HMI sets to true after uploading all options
pub const CONFIG_READY: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 1 },
    name: "CONFIG READY",
    description: "Configuration upload complete",
};

/// Is the current operation cancellable (should display cancel button)
pub const IS_OP_CANCELLABLE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 9 },
    name: "IS OP CANCELLABLE",
    description: "Cancel is available for the current operation",
};

/// Start cycle latch
pub const START_CYCLE_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 10 },
    name: "START CYCLE LATCH",
    description: "Start cycle",
};

/// Home latch
pub const HOME_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 11 },
    name: "HOME LATCH",
    description: "Home axes",
};

/// Is currently homing
pub const IS_HOMING: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 12 },
    name: "IS HOMING",
    description: "Homing in progress",
};

/// Is homed
pub const IS_HOMED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 13 },
    name: "IS HOMED",
    description: "Machine homed",
};

/// At start position
pub const AT_START: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 14 },
    name: "AT START",
    description: "At cycle start position",
};

/// Go to start latch
pub const GO_TO_START_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 15 },
    name: "GO TO START LATCH",
    description: "Move to start position",
};

/// Job is active
pub const JOB_ACTIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 16 },
    name: "JOB ACTIVE",
    description: "Cycle running",
};

/// Cancel operation latch
pub const CANCEL_OPERATION_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 17 },
    name: "CANCEL OPERATION LATCH",
    description: "Cancel current operation",
};

/// HMI commands estop
pub const FORCE_ESTOP: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 18 },
    name: "FORCE ESTOP",
    description: "Trigger emergency stop",
};

/// If false, it will be in sim mode
pub const WELD_ENABLE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 19 },
    name: "WELD ENABLE",
    description: "Enable welding output",
};

/// Weld signal
pub const WELD_SIGNAL: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 20 },
    name: "WELD SIGNAL",
    description: "Weld signal output",
};
/// When true, gas will be forced on. For purge or pre-flow
pub const ENABLE_GAS_OUTPUT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 21 },
    name: "ENABLE GAS OUTPUT",
    description: "Force gas on for purge or pre-flow",
};
/// Error state
pub const ERROR: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 22 },
    name: "ERROR",
    description: "Error active",
};
/// latch: command left fingers up
pub const COMMAND_LF_UP_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 33 },
    name: "COMMAND LF UP LATCH",
    description: "Raise left fingers",
};
/// latch: command left fingers down
pub const COMMAND_LF_DOWN_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 34 },
    name: "COMMAND LF DOWN LATCH",
    description: "Lower left fingers",
};
/// latch: command right fingers up
pub const COMMAND_RF_UP_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 35 },
    name: "COMMAND RF UP LATCH",
    description: "Raise right fingers",
};
/// latch: command right fingers down
pub const COMMAND_RF_DOWN_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 36 },
    name: "COMMAND RF DOWN LATCH",
    description: "Lower right fingers",
};

/// When true, cycle will use CYCLE_AVC_VREF to manage Z height, else will use CYCLE_Z_STATIC_OFFSET
pub const CYCLE_USE_AVC: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 37 },
    name: "CYCLE USE AVC",
    description: "Use AVC for Z height control",
};

/// TODO
pub const CYCLE_USE_TOUCH_RETRACT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 38 },
    name: "CYCLE USE TOUCH RETRACT",
    description: "Use touch retract for Z height",
};

/// Jog X axis in positive direction
pub const JOG_X_AXIS_POSITIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 40 },
    name: "JOG X AXIS POSITIVE",
    description: "Jog X in the positive direction",
};

/// Jog X axis in negative direction
pub const JOG_X_AXIS_NEGATIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 41 },
    name: "JOG X AXIS NEGATIVE",
    description: "Jog X in the negative direction",
};

/// Jog Y axis in positive direction
pub const JOG_Y_AXIS_POSITIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 42 },
    name: "JOG Y AXIS POSITIVE",
    description: "Jog Y in the positive direction",
};

/// Jog Y axis in negative direction
pub const JOG_Y_AXIS_NEGATIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 43 },
    name: "JOG Y AXIS NEGATIVE",
    description: "Jog Y in the negative direction",
};

/// Jog Z axis in positive direction
pub const JOG_Z_AXIS_POSITIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 44 },
    name: "JOG Z AXIS POSITIVE",
    description: "Jog Z in the positive direction",
};

/// Jog Z axis in negative direction
pub const JOG_Z_AXIS_NEGATIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 45 },
    name: "JOG Z AXIS NEGATIVE",
    description: "Jog Z in the negative direction",
};

/// Jog W axis in positive direction
pub const JOG_W_AXIS_POSITIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 46 },
    name: "JOG W AXIS POSITIVE",
    description: "Jog W in the positive direction",
};

/// Jog W axis in negative direction
pub const JOG_W_AXIS_NEGATIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 47 },
    name: "JOG W AXIS NEGATIVE",
    description: "Jog W in the negative direction",
};

/// Direction to home the X axis motor. True is positive
pub const AXIS_X_HOME_DIRECTION_POSITIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 50 },
    name: "AXIS X HOME DIRECTION",
    description: "X axis homes toward positive",
};

/// Direction to home the Y axis motor. True is positive
pub const AXIS_Y_HOME_DIRECTION_POSITIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 51 },
    name: "AXIS Y HOME DIRECTION",
    description: "Y axis homes toward positive",
};

/// Direction to home the Z axis motor. True is positive
pub const AXIS_Z_HOME_DIRECTION_POSITIVE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 52 },
    name: "AXIS Z HOME DIRECTION",
    description: "Z axis homes toward positive",
};

/// Machine uses Y axis
pub const USES_Y_AXIS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 65 },
    name: "USES Y AXIS",
    description: "Enable Y axis",
};

/// Machine uses Z axis
pub const USES_Z_AXIS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 66 },
    name: "USES Z AXIS",
    description: "Enable Z axis",
};

/// Machine uses W axis
pub const USES_W_AXIS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 67 },
    name: "USES W AXIS",
    description: "Enable W axis",
};

/// Latch to command X axis go to position
pub const X_AXIS_GO_TO_POSITION_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 68 },
    name: "X AXIS GO TO POSITION LATCH",
    description: "Move X to target position",
};

/// Latch to command Y axis go to position
pub const Y_AXIS_GO_TO_POSITION_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 69 },
    name: "Y AXIS GO TO POSITION LATCH",
    description: "Move Y to target position",
};

/// Latch to command Z axis go to position
pub const Z_AXIS_GO_TO_POSITION_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 70 },
    name: "Z AXIS GO TO POSITION LATCH",
    description: "Move Z to target position",
};

/// True when X axis is homed
pub const X_AXIS_IS_HOMED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 71 },
    name: "X AXIS IS HOMED",
    description: "X axis homed",
};

/// True when Y axis is homed
pub const Y_AXIS_IS_HOMED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 72 },
    name: "Y AXIS IS HOMED",
    description: "Y axis homed",
};

/// True when Z axis is homed
pub const Z_AXIS_IS_HOMED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 73 },
    name: "Z AXIS IS HOMED",
    description: "Z axis homed",
};

/// True means welder runs in simulate mode
pub const WELDER_SIMULATE_MODE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 74 },
    name: "WELDER SIMULATE MODE",
    description: "Simulate welder output",
};
/// UNUSED True means the touch retract coil on the welder is currently enabled. This is the REQUEST. Not sense feedback.
pub const TOUCH_RETRACT_ENABLED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 75 },
    name: "TOUCH RETRACT ENABLED",
    description: "Touch retract output enabled (unused)",
};
/// Like the {}_AXIS_GO_TO_POSITION_LATCH's but instead of an absolute position, it jogs that far from current
pub const W_AXIS_GO_TO_RELATIVE_POSITION_LATCH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 76 },
    name: "W AXIS GO TO RELATIVE POSITION LATCH",
    description: "Move W by a relative distance",
};



// ============================================================================
// DISCRETE INPUTS
// ============================================================================

/// Discrete input (TODO: define purpose)
pub const FORGOR: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 0 },
    name: "FORGOR",
    description: "Unused discrete input",
};
/// true when an arc is commanded. This happens even in simulate mode, to show when it WOULD be active
pub const WELDER_ARC_COMMANDED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2 },
    name: "WELDER ARC COMMANDED",
    description: "Arc command active",
};
/// true when an arc is valid. this is from the miller and only happens when there is a REAL ARC
pub const WELDER_ARC_VALID: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 3 },
    name: "WELDER ARC VALID",
    description: "Arc detected",
};
/// true when estop is engaged
pub const IN_ESTOP: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 4 },
    name: "IN ESTOP",
    description: "Emergency stop engaged",
};
/// true when the mandrel latch is sensed to be closed
pub const MANDREL_LATCH_CLOSED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 5 },
    name: "MANDREL LATCH CLOSED",
    description: "Mandrel latch closed",
};
/// Are the left fingers currently commanded down
pub const LF_COMMANDED_DOWN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 6 },
    name: "LF COMMANDED DOWN",
    description: "Left fingers commanded down",
};
/// Are the right fingers currently commanded down
pub const RF_COMMANDED_DOWN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 7 },
    name: "RF COMMANDED DOWN",
    description: "Right fingers commanded down",
};
/// true when clearcore wants to enable touch retract
pub const TOUCH_RETRACT_REQUESTED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 8 },
    name: "TOUCH RETRACT REQUESTED",
    description: "Touch retract requested",
};

// ============================================================================
// INPUT REGISTERS
// ============================================================================

/// Serial number low word
pub const SERIAL_NUMBER_LOW: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 0 },
    name: "SERIAL NUMBER LOW",
    description: "Clearcore serial number (low word of 2 part register)",
};

/// Serial number high word
pub const SERIAL_NUMBER_HIGH: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 1 },
    name: "SERIAL NUMBER HIGH",
    description: "Clearcore serial number (high word of 2 part register)",
};
/// hundredths of a volt| this shows the voltage on the arc with avc-l/o gating, a convolution, and correction factor
pub const MEASURED_AVC_VOLTAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 10 },
    name: "MEASURED AVC VOLTAGE",
    description: "Measured arc voltage (V)",
};
/// hundredths of an amp| this shows the current on the arc with avc-l/o gating, a convolution, and correction factor
pub const MEASURED_AVC_CURRENT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 11 },
    name: "MEASURED AVC CURRENT",
    description: "Measured arc current (A)",
};

// ============================================================================
// HOLDING REGISTERS
// ============================================================================

/// port to send UDP logs to. Logs will be sent to whoever connects to modbus on this port. 0 means no logging
pub const UDP_LOG_PORT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6 },
    name: "UDP LOG PORT",
    description: "UDP log port",
};
/// Current position of carriage (hundredths of an inch)
pub const X_AXIS_POSITION: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 8 },
    name: "X AXIS POSITION",
    description: "X axis position (in)",
};
/// Current position of carriage (hundredths of an inch)
pub const Y_AXIS_POSITION: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 9 },
    name: "Y AXIS POSITION",
    description: "Y axis position (in)",
};
/// Current position of carriage (hundredths of an inch)
pub const Z_AXIS_POSITION: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 10 },
    name: "Z AXIS POSITION",
    description: "Z axis position (in)",
};
/// Distance from 0 to start weld (hundredths of an inch)
pub const CYCLE_START_POS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 13 },
    name: "CYCLE START POS",
    description: "Weld start position (in)",
};

/// Distance from 0 to finish weld (hundredths of an inch)
pub const CYCLE_END_POS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 14 },
    name: "CYCLE END POS",
    description: "Weld end position (in)",
};

/// Distance from 0 to park after weld cycle (hundredths of an inch)
pub const CYCLE_PARK_POS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 15 },
    name: "CYCLE PARK POS",
    description: "Park position after cycle (in)",
};

/// Speed to weld (hundredths of an inch per minute)
pub const CYCLE_WELD_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 16 },
    name: "CYCLE WELD SPEED",
    description: "Weld travel speed (in/min)",
};

/// Speed to extrude wire while welding (hundredths of an inch per minute)
pub const CYCLE_WIRE_FEED_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 18 },
    name: "CYCLE WIRE FEED SPEED",
    description: "Wire feed speed during weld (in/min)",
};

/// Currently commanded jogging speed for X axis (hundredths of an inch per minute)
pub const AXIS_X_COMMANDED_JOG_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 20 },
    name: "AXIS X COMMANDED JOG SPEED",
    description: "Current X jog speed (in/min)",
};

/// Currently commanded jogging speed for Y axis (hundredths of an inch per minute)
pub const AXIS_Y_COMMANDED_JOG_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 21 },
    name: "AXIS Y COMMANDED JOG SPEED",
    description: "Current Y jog speed (in/min)",
};

/// Currently commanded jogging speed for Z axis (hundredths of an inch per minute)
pub const AXIS_Z_COMMANDED_JOG_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 22 },
    name: "AXIS Z COMMANDED JOG SPEED",
    description: "Current Z jog speed (in/min)",
};

/// Currently commanded jogging speed for W axis (hundredths of an inch per minute)
pub const AXIS_W_COMMANDED_JOG_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 23 },
    name: "AXIS W COMMANDED JOG SPEED",
    description: "Current W jog speed (in/min)",
};

/// Speed to use when homing X axis (hundredths of an inch per minute)
pub const AXIS_X_HOMING_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 24 },
    name: "AXIS X HOMING SPEED",
    description: "X axis homing speed (in/min)",
};

/// Speed to use when homing Y axis (hundredths of an inch per minute)
pub const AXIS_Y_HOMING_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 25 },
    name: "AXIS Y HOMING SPEED",
    description: "Y axis homing speed (in/min)",
};

/// Speed to use when homing Z axis (hundredths of an inch per minute)
pub const AXIS_Z_HOMING_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 26 },
    name: "AXIS Z HOMING SPEED",
    description: "Z axis homing speed (in/min)",
};

/// Distance to move X axis away from hardware limit after homing (hundredths of an inch)
pub const AXIS_X_HOMING_OFFSET: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 27 },
    name: "AXIS X HOMING OFFSET",
    description: "X axis homing offset (in)",
};

/// Distance to move Y axis away from hardware limit after homing (hundredths of an inch)
pub const AXIS_Y_HOMING_OFFSET: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 28 },
    name: "AXIS Y HOMING OFFSET",
    description: "Y axis homing offset (in)",
};

/// Distance to move Z axis away from hardware limit after homing (hundredths of an inch)
pub const AXIS_Z_HOMING_OFFSET: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 29 },
    name: "AXIS Z HOMING OFFSET",
    description: "Z axis homing offset (in)",
};

/// Progress on current job (hundredths of percent, 0..10000)
pub const CYCLE_PROGRESS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 30 },
    name: "CYCLE PROGRESS",
    description: "Cycle progress (%)",
};

/// Target voltage for AVC (hundredth of volt)
pub const CYCLE_AVC_VREF: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 31 },
    name: "AVC VREF",
    description: "Target arc voltage (V)",
};

/// offset to use for Z axis when CYCLE_USE_AVC is false. measured as distance from top of travel (hundredths of an inch)
pub const CYCLE_Z_STATIC_OFFSET: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 32 },
    name: "Z STATIC OFFSET",
    description: "Fixed Z height when AVC is off (in)",
};

/// speed to move x axis in cycle when not actively welding (hundredths of an inch per minute)
pub const CYCLE_REPOSITION_SPEED_X: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 33 },
    name: "REPOSITION SPEED X",
    description: "X axis travel speed between welds (in/min)",
};

/// speed to move y axis in cycle when not actively welding (hundredths of an inch per minute)
pub const CYCLE_REPOSITION_SPEED_Y: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 34 },
    name: "REPOSITION SPEED Y",
    description: "Y axis travel speed between welds (in/min)",
};

/// speed to move z axis in cycle when not actively welding (hundredths of an inch per minute)
pub const CYCLE_REPOSITION_SPEED_Z: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 35 },
    name: "REPOSITION SPEED Z",
    description: "Z axis travel speed between welds (in/min)",
};
/// hundredths of an inch| offset to use for Z axis when torch is up after a cycle. measured as distance from top of travel
pub const CYCLE_AXIS_Z_TORCH_UP_OFFSET: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 36 },
    name: "AXIS Z TORCH UP OFFSET",
    description: "Torch up height after cycle (in)",
};

/// hundredths of an inch| distance to move the Z axis at reposition speed before slowing down during touch retract. measured as distance from top of travel
pub const CYCLE_TOUCH_RETRACT_REPOSITION_DISTANCE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 37 },
    name: "TOUCH RETRACT REPOSITION DIST",
    description: "Touch retract fast travel distance (in)",
};

/// hundredths of an inch per minute| probing speed after reaching CYCLE_TOUCH_RETRACT_REPOSITION_DISTANCE during touch retract
pub const CYCLE_TOUCH_RETRACT_PROBE_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 38 },
    name: "TOUCH RETRACT PROBE SPEED",
    description: "Touch retract probe speed (in/min)",
};

/// hundredths of an inch| final height when using touch retract measured from the part (only used with touch retract mode)
pub const CYCLE_TOUCH_RETRACT_FINAL_HEIGHT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 39 },
    name: "TOUCH RETRACT FINAL HEIGHT",
    description: "Touch retract final height (in)",
};

/// unitless factor in thousandths| AVC correction strength multiplier. 1000 == 1.0x
pub const CYCLE_AVC_CORRECTION_STRENGTH_MULTIPLIER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 40 },
    name: "AVC CORRECTION STRENGTH MULTIPLIER",
    description: "AVC correction strength multiplier (x)",
};

/// hundredths of an inch per minute| speed limit for z axis while making AVC correction moves
pub const CYCLE_AVC_TRAVEL_SPEED_Z: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 41 },
    name: "AVC TRAVEL SPEED Z",
    description: "AVC correction Z speed limit (in/min)",
};
/// unitless factor| This number divided by 10000 is multiplied by the raw adc voltage reading to get the actual arc voltage
pub const ARC_VOLTAGE_CALIBRATION_FACTOR: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 60 },
    name: "ARC VOLTAGE ADC CORRECTION",
    description: "Arc voltage calibration factor (unitless)",
};
/// unitless factor| This number divided by 10000 is multiplied by the raw adc current reading to get the actual arc current
pub const ARC_CURRENT_CALIBRATION_FACTOR: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 61 },
    name: "ARC CURRENT ADC CORRECTION",
    description: "Arc current calibration factor (unitless)",
};
/// u32 where combined value over 10^9 is inches per step. range: 0..4.294967296
pub const INCHES_PER_STEP_X_AXIS_LOWER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 64 },
    name: "X AXIS SCALING",
    description: "X axis scale (in/step)",
};

/// See HUNDREDTHS_PER_STEP_X_AXIS_LOWER
pub const INCHES_PER_STEP_X_AXIS_UPPER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 65 },
    name: "X AXIS SCALING UPPER",
    description: "X axis scale (upper word)",
};

/// u32 where combined value over 10^9 is inches per step. range: 0..4.294967296
pub const INCHES_PER_STEP_Y_AXIS_LOWER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 66 },
    name: "Y AXIS SCALING",
    description: "Y axis scale (in/step)",
};

/// See HUNDREDTHS_PER_STEP_Y_AXIS_LOWER
pub const INCHES_PER_STEP_Y_AXIS_UPPER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 67 },
    name: "Y AXIS SCALING UPPER",
    description: "Y axis scale (upper word)",
};

/// u32 where combined value over 10^9 is inches per step. range: 0..4.294967296
pub const INCHES_PER_STEP_Z_AXIS_LOWER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 68 },
    name: "Z AXIS SCALING",
    description: "Z axis scale (in/step)",
};

/// See HUNDREDTHS_PER_STEP_Z_AXIS_LOWER
pub const INCHES_PER_STEP_Z_AXIS_UPPER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 69 },
    name: "Z AXIS SCALING UPPER",
    description: "Z axis scale (upper word)",
};

/// u32 where combined value over 10^9 is inches per step. range: 0..4.294967296
pub const INCHES_PER_STEP_W_AXIS_LOWER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 70 },
    name: "W AXIS SCALING",
    description: "W axis scale (in/step)",
};

/// See HUNDREDTHS_PER_STEP_W_AXIS_LOWER
pub const INCHES_PER_STEP_W_AXIS_UPPER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 71 },
    name: "W AXIS SCALING UPPER",
    description: "W axis scale (upper word)",
};

/// Minimum position for X axis (hundredths of an inch)
pub const MIN_POS_X_AXIS_HUNDREDTHS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 72 },
    name: "MIN POS X AXIS",
    description: "X axis minimum position (in)",
};

/// Maximum position for X axis (hundredths of an inch)
pub const MAX_POS_X_AXIS_HUNDREDTHS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 73 },
    name: "MAX POS X AXIS",
    description: "X axis maximum position (in)",
};

/// Maximum velocity for X axis (hundredths of an inch per minute)
pub const MAX_VEL_X_AXIS_HUNDREDTHS_PER_MINUTE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 74 },
    name: "MAX VEL X AXIS",
    description: "X axis maximum speed (in/min)",
};

/// Maximum acceleration for X axis (hundredths of an inch per minute per second)
pub const MAX_ACCEL_X_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 75 },
    name: "MAX ACCEL X AXIS",
    description: "X axis maximum acceleration (in/min/s)",
};

/// Minimum position for Y axis (hundredths of an inch)
pub const MIN_POS_Y_AXIS_HUNDREDTHS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 76 },
    name: "MIN POS Y AXIS",
    description: "Y axis minimum position (in)",
};

/// Maximum position for Y axis (hundredths of an inch)
pub const MAX_POS_Y_AXIS_HUNDREDTHS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 77 },
    name: "MAX POS Y AXIS",
    description: "Y axis maximum position (in)",
};

/// Maximum velocity for Y axis (hundredths of an inch per minute)
pub const MAX_VEL_Y_AXIS_HUNDREDTHS_PER_MINUTE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 78 },
    name: "MAX VEL Y AXIS",
    description: "Y axis maximum speed (in/min)",
};

/// Maximum acceleration for Y axis (hundredths of an inch per minute per second)
pub const MAX_ACCEL_Y_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 79 },
    name: "MAX ACCEL Y AXIS",
    description: "Y axis maximum acceleration (in/min/s)",
};

/// Minimum position for Z axis (hundredths of an inch)
pub const MIN_POS_Z_AXIS_HUNDREDTHS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 80 },
    name: "MIN POS Z AXIS",
    description: "Z axis minimum position (in)",
};

/// Maximum position for Z axis (hundredths of an inch)
pub const MAX_POS_Z_AXIS_HUNDREDTHS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 81 },
    name: "MAX POS Z AXIS",
    description: "Z axis maximum position (in)",
};

/// Maximum velocity for Z axis (hundredths of an inch per minute)
pub const MAX_VEL_Z_AXIS_HUNDREDTHS_PER_MINUTE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 82 },
    name: "MAX VEL Z AXIS",
    description: "Z axis maximum speed (in/min)",
};

/// Maximum acceleration for Z axis (hundredths of an inch per minute per second)
pub const MAX_ACCEL_Z_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 83 },
    name: "MAX ACCEL Z AXIS",
    description: "Z axis maximum acceleration (in/min/s)",
};

/// Maximum velocity for W axis (hundredths of an inch per minute)
pub const MAX_VEL_W_AXIS_HUNDREDTHS_PER_MINUTE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 84 },
    name: "MAX VEL W AXIS",
    description: "W axis maximum speed (in/min)",
};

/// Maximum acceleration for W axis (hundredths of an inch per minute per second)
pub const MAX_ACCEL_W_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 85 },
    name: "MAX ACCEL W AXIS",
    description: "W axis maximum acceleration (in/min/s)",
};

/// Target position for X axis go-to command (hundredths of an inch)
pub const X_AXIS_GO_TO_POSITION: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 96 },
    name: "X AXIS GO TO POSITION",
    description: "X axis target position (in)",
};

/// Target position for Y axis go-to command (hundredths of an inch)
pub const Y_AXIS_GO_TO_POSITION: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 97 },
    name: "Y AXIS GO TO POSITION",
    description: "Y axis target position (in)",
};

/// Target position for Z axis go-to command (hundredths of an inch)
pub const Z_AXIS_GO_TO_POSITION: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 98 },
    name: "Z AXIS GO TO POSITION",
    description: "Z axis target position (in)",
};

/// Default jog speed for X axis (hundredths of an inch per minute)
pub const AXIS_X_DEFAULT_JOG_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 99 },
    name: "AXIS X DEFAULT JOG SPEED",
    description: "Default X jog speed (in/min)",
};

/// Default jog speed for Y axis (hundredths of an inch per minute)
pub const AXIS_Y_DEFAULT_JOG_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 100 },
    name: "AXIS Y DEFAULT JOG SPEED",
    description: "Default Y jog speed (in/min)",
};

/// Default jog speed for Z axis (hundredths of an inch per minute)
pub const AXIS_Z_DEFAULT_JOG_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 101 },
    name: "AXIS Z DEFAULT JOG SPEED",
    description: "Default Z jog speed (in/min)",
};

/// Default jog speed for W axis (hundredths of an inch per minute)
pub const AXIS_W_DEFAULT_JOG_SPEED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 102 },
    name: "AXIS W DEFAULT JOG SPEED",
    description: "Default W jog speed (in/min)",
};
/// hundredths of an inch + 32,768| relative go to position for w axis
pub const W_AXIS_RELATIVE_GO_TO_POTION: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 103 },
    name: "W AXIS RELATIVE GO TO POSITION",
    description: "W axis relative move distance (in)",
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
    ENABLE_GAS_OUTPUT,
    IN_ESTOP,
    ERROR,
    COMMAND_LF_UP_LATCH,
    COMMAND_LF_DOWN_LATCH,
    COMMAND_RF_UP_LATCH,
    COMMAND_RF_DOWN_LATCH,
    CYCLE_USE_AVC,
    CYCLE_USE_TOUCH_RETRACT,
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
    X_AXIS_GO_TO_POSITION_LATCH,
    Y_AXIS_GO_TO_POSITION_LATCH,
    Z_AXIS_GO_TO_POSITION_LATCH,
    X_AXIS_IS_HOMED,
    Y_AXIS_IS_HOMED,
    Z_AXIS_IS_HOMED,
    WELDER_SIMULATE_MODE,
    TOUCH_RETRACT_ENABLED,
    W_AXIS_GO_TO_RELATIVE_POSITION_LATCH,

    FORGOR,
    WELDER_ARC_COMMANDED,
    WELDER_ARC_VALID,
    LF_COMMANDED_DOWN,
    RF_COMMANDED_DOWN,
    TOUCH_RETRACT_REQUESTED,

    SERIAL_NUMBER_LOW,
    SERIAL_NUMBER_HIGH,
    MEASURED_AVC_VOLTAGE,
    MEASURED_AVC_CURRENT,

    UDP_LOG_PORT,
    X_AXIS_POSITION,
    Y_AXIS_POSITION,
    Z_AXIS_POSITION,
    CYCLE_START_POS,
    CYCLE_END_POS,
    CYCLE_PARK_POS,
    CYCLE_WELD_SPEED,
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
    CYCLE_AVC_VREF,
    CYCLE_Z_STATIC_OFFSET,
    CYCLE_REPOSITION_SPEED_X,
    CYCLE_REPOSITION_SPEED_Y,
    CYCLE_REPOSITION_SPEED_Z,
    CYCLE_AXIS_Z_TORCH_UP_OFFSET,
    CYCLE_TOUCH_RETRACT_REPOSITION_DISTANCE,
    CYCLE_TOUCH_RETRACT_PROBE_SPEED,
    CYCLE_TOUCH_RETRACT_FINAL_HEIGHT,
    CYCLE_AVC_CORRECTION_STRENGTH_MULTIPLIER,
    CYCLE_AVC_TRAVEL_SPEED_Z,
    ARC_VOLTAGE_CALIBRATION_FACTOR,
    ARC_CURRENT_CALIBRATION_FACTOR,
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
    X_AXIS_GO_TO_POSITION,
    Y_AXIS_GO_TO_POSITION,
    Z_AXIS_GO_TO_POSITION,
    AXIS_X_DEFAULT_JOG_SPEED,
    AXIS_Y_DEFAULT_JOG_SPEED,
    AXIS_Z_DEFAULT_JOG_SPEED,
    AXIS_W_DEFAULT_JOG_SPEED,
    W_AXIS_RELATIVE_GO_TO_POTION,
];


pub fn get_clearcore_register_metadata(register_name: &str) -> Option<&'static RegisterMetadata> {
    static REGISTER_MAP: std::sync::OnceLock<std::collections::HashMap<&'static str, &'static RegisterMetadata>> = std::sync::OnceLock::new();

    let map = REGISTER_MAP.get_or_init(|| {
        CLEARCORE_REGISTERS.iter().map(|r| (r.name, r)).collect()
    });

    map.get(register_name).copied()
}
