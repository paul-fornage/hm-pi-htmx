use crate::analog_register::AnalogRegisterInfo;
use crate::modbus::RegisterMetadata;
use crate::plc::plc_register_definitions::*;



const CLEARCORE_STATIC_CONFIG_COILS: &[RegisterMetadata] = &[
    AXIS_X_HOME_DIRECTION_POSITIVE,
    AXIS_Y_HOME_DIRECTION_POSITIVE,
    AXIS_Z_HOME_DIRECTION_POSITIVE,
    USES_Y_AXIS,
    USES_Z_AXIS,
    USES_W_AXIS,
];



const CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS: &[AnalogRegisterInfo] = &[
    AnalogRegisterInfo::new(&AXIS_X_HOMING_SPEED, "in/min", 2, 0),
    AnalogRegisterInfo::new(&AXIS_Y_HOMING_SPEED, "in/min", 2, 0),
    AnalogRegisterInfo::new(&AXIS_Z_HOMING_SPEED, "in/min", 2, 0),
    AnalogRegisterInfo::new(&AXIS_X_HOMING_OFFSET, "in", 2, 0),
    AnalogRegisterInfo::new(&AXIS_Y_HOMING_OFFSET, "in", 2, 0),
    AnalogRegisterInfo::new(&AXIS_Z_HOMING_OFFSET, "in", 2, 0),
    AnalogRegisterInfo::new(&MIN_POS_X_AXIS_HUNDREDTHS, "in", 2, 0),
    AnalogRegisterInfo::new(&MAX_POS_X_AXIS_HUNDREDTHS, "in", 2, 0),
    AnalogRegisterInfo::new(&MAX_VEL_X_AXIS_HUNDREDTHS_PER_MINUTE, "in/min", 2, 0),
    AnalogRegisterInfo::new(&MAX_ACCEL_X_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND, "in/min×s", 2, 0),
    AnalogRegisterInfo::new(&MIN_POS_Y_AXIS_HUNDREDTHS, "in", 2, 0),
    AnalogRegisterInfo::new(&MAX_POS_Y_AXIS_HUNDREDTHS, "in", 2, 0),
    AnalogRegisterInfo::new(&MAX_VEL_Y_AXIS_HUNDREDTHS_PER_MINUTE, "in/min", 2, 0),
    AnalogRegisterInfo::new(&MAX_ACCEL_Y_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND, "in/min×s", 2, 0),
    AnalogRegisterInfo::new(&MIN_POS_Z_AXIS_HUNDREDTHS, "in", 2, 0),
    AnalogRegisterInfo::new(&MAX_POS_Z_AXIS_HUNDREDTHS, "in", 2, 0),
    AnalogRegisterInfo::new(&MAX_VEL_Z_AXIS_HUNDREDTHS_PER_MINUTE, "in/min", 2, 0),
    AnalogRegisterInfo::new(&MAX_ACCEL_Z_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND, "in/min×s", 2, 0),
    AnalogRegisterInfo::new(&MAX_VEL_W_AXIS_HUNDREDTHS_PER_MINUTE, "in/min", 2, 0),
    AnalogRegisterInfo::new(&MAX_ACCEL_W_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND, "in/min×s", 2, 0),
];

const CLEARCORE_STATIC_CONFIG_DWORD_HREGS: &[RegisterMetadata] = &[
    HUNDREDTHS_PER_STEP_X_AXIS_LOWER,
    HUNDREDTHS_PER_STEP_X_AXIS_UPPER,
    HUNDREDTHS_PER_STEP_Y_AXIS_LOWER,
    HUNDREDTHS_PER_STEP_Y_AXIS_UPPER,
    HUNDREDTHS_PER_STEP_Z_AXIS_LOWER,
    HUNDREDTHS_PER_STEP_Z_AXIS_UPPER,
    HUNDREDTHS_PER_STEP_W_AXIS_LOWER,
    HUNDREDTHS_PER_STEP_W_AXIS_UPPER,
]