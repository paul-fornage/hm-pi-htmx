use crate::miller::miller_register_definitions as miller;
use crate::plc::plc_register_definitions as cc;
use crate::modbus::RegisterMetadata;



pub const ADJUSTABLE_REGISTERS: &[RegisterMetadata] = &[
    miller::PRESET_MIN_AMPERAGE,
    miller::ARC_START_AMPERAGE,
    miller::ARC_START_TIME,
    miller::ARC_START_SLOPE_TIME,
    miller::ARC_START_AC_TIME,
    miller::HOT_START_TIME,
    miller::AC_EN_AMPERAGE,
    miller::AC_EP_AMPERAGE,
    miller::AC_BALANCE,
    miller::AC_FREQUENCY,
    miller::WELD_AMPERAGE,
    miller::PULSER_PPS,
    miller::PULSER_PEAK_TIME,
    miller::PREFLOW_TIME,
    miller::INITIAL_AMPERAGE,
    miller::INITIAL_TIME,
    miller::INITIAL_SLOPE_TIME,
    miller::MAIN_TIME,
    miller::FINAL_SLOPE_TIME,
    miller::FINAL_AMPERAGE,
    miller::FINAL_TIME,
    miller::HOT_WIRE_VOLTAGE,

    cc::CYCLE_START_POS,
    cc::CYCLE_END_POS,
    cc::CYCLE_PARK_POS,
    cc::CYCLE_WELD_SPEED,
    cc::CYCLE_REPOSITION_SPEED_X,
    cc::CYCLE_REPOSITION_SPEED_Y,
    cc::CYCLE_REPOSITION_SPEED_Z,
    cc::CYCLE_WIRE_FEED_SPEED,
    cc::CYCLE_AVC_VREF,
    cc::CYCLE_AVC_CORRECTION_STRENGTH_MULTIPLIER,
    cc::CYCLE_AVC_TRAVEL_SPEED_Z,
    cc::CYCLE_Z_STATIC_OFFSET,
    cc::CYCLE_AXIS_Z_TORCH_UP_OFFSET,
    cc::CYCLE_TOUCH_RETRACT_REPOSITION_DISTANCE,
    cc::CYCLE_TOUCH_RETRACT_PROBE_SPEED,
    cc::CYCLE_TOUCH_RETRACT_FINAL_HEIGHT,
];
