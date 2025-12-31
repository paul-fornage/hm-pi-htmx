use std::num::NonZeroU16;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::error_targeted;



fn unwrap_opt_nz_u16(v: Option<NonZeroU16>) -> u16 { v.unwrap().get() }


pub trait AsModbusRegisterValue {
    fn as_modbus_register_value(&self) -> u16;
}


pub struct ClampedInclusiveU16<const MIN: u16, const MAX: u16>(u16);
impl<const MIN: u16, const MAX: u16> ClampedInclusiveU16<MIN, MAX> {
    pub fn try_from_u16(val: u16) -> Option<Self> {
        if val > MAX { None } else if val < MIN { None } else { Some(Self(val)) }
    }
    pub fn value(&self) -> u16 { self.0 }
}



// *Hot Wire Voltage, 5-20, Res: 1V
pub struct HotWireVoltage(pub ClampedInclusiveU16<5, 20>);


// *Postflow Time, 0(Off) - 50S & Auto(51), Res: 1Sec
#[repr(u8)]
pub enum PostFlowTime {
    Off,
    Manual(u8),
    Auto,
}
impl PostFlowTime {
    const OFF_VAL: u16 = 0;
    const MIN_MANUAL_VAL: u16 = Self::OFF_VAL + 1;
    const MAX_MANUAL_VAL: u16 = Self::AUTO_VAL - 1;
    const AUTO_VAL: u16 = 51;
}
impl AsModbusRegisterValue for PostFlowTime{
    fn as_modbus_register_value(&self) -> u16 {
        match self {
            Self::Off => Self::OFF_VAL,
            Self::Manual(v) => {
                let val = *v as u16;
                if val > Self::MAX_MANUAL_VAL {
                    error_targeted!(MODBUS, "PostFlowTime value {} is out of range [{}..={}]",
                        val, Self::MIN_MANUAL_VAL, Self::MAX_MANUAL_VAL);
                    Self::MAX_MANUAL_VAL
                } else if val < Self::MIN_MANUAL_VAL {
                    error_targeted!(MODBUS, "PostFlowTime value {} is out of range [{}..={}]",
                        val, Self::MIN_MANUAL_VAL, Self::MAX_MANUAL_VAL);
                    Self::MIN_MANUAL_VAL
                } else { val }
            },
            Self::Auto => Self::AUTO_VAL,
        }
    }
}


pub struct NonZeroBoundedU16<const MAX: u16>(Option<NonZeroU16>);
impl<const MAX: u16> NonZeroBoundedU16<MAX> {

    pub const ZERO_VALUE: Self = Self(None);
    pub fn try_from_u16(val: u16) -> Option<Self> {
        match val{
            0 => None,
            v if v > MAX => { None },
            // safety: we just checked to make sure it isn't 0
            v => unsafe { Some(Self(Some(NonZeroU16::new_unchecked(val)))) },
        }
    }

    /// val must be non-zero and less than or equal to max.
    pub unsafe fn new_unchecked(val: u16) -> Self{
        Self(Some(NonZeroU16::new_unchecked(val)))
    }
}
impl<const MAX: u16> AsModbusRegisterValue for NonZeroBoundedU16<MAX> {
    fn as_modbus_register_value(&self) -> u16 { unwrap_opt_nz_u16(self.0) }
}

// from 0 (off) to MAX in tenths of a second
pub struct WavePhaseDuration<const MAX: u16>(pub NonZeroBoundedU16<MAX>);


// *Pulser - Background Amperage, 5-95%, Res: 1%
pub struct PulserBackgroundAmps(pub ClampedInclusiveU16<5, 95>);
impl AsModbusRegisterValue for PulserBackgroundAmps {
    fn as_modbus_register_value(&self) -> u16 { self.0.value() }
}


// *Pulser - Peak Time, 5-95%, Res: 1%
pub struct PulserPeakTime(pub ClampedInclusiveU16<5, 95>);
impl AsModbusRegisterValue for PulserPeakTime {
    fn as_modbus_register_value(&self) -> u16 { self.0.value() }
}

/*
"*Pulser - Pulses Per Second (PPS)
Range: 0(Off) – 50000 / 5000 Power Source Dependent,
Resolution: 0.1 Hertz
Can be set to a default value when writing a TRUE to coil 18 Pulser Enable and PPS is found at 0(Off).
Writing a non “0” value will set coil 18 Pulser Enable to TRUE.
Writing a “0” value will set coil 18 Pulser Enable to FALSE.
Dependent on configuration of the slave, the slave may or may not retain the PPS non “0” value."
*/
const MAX_PULSER_FREQUENCY_HERTZ: u16 = 5000;
const MAX_PULSER_RAW: u16 = MAX_PULSER_FREQUENCY_HERTZ * 10;
pub struct PulseFrequency(pub NonZeroBoundedU16<MAX_PULSER_RAW>);
impl PulseFrequency {
    pub fn as_hertz_option(&self) -> Option<f32> { self.0.0.map(|v| v.get() as f32 / 10.0) }
    pub fn try_from_f32_hertz(v: f32) -> Option<Self> {
        let val = (v * 10.0).round() as i32;
        match val{
            negative if negative < 0 => None,
            0 => Some(Self(NonZeroBoundedU16::ZERO_VALUE)),
            too_high if too_high > MAX_PULSER_RAW as i32 => None,
            value => unsafe {
                // safety: We have checked the value is non-zero and within bounds
                Some(Self(NonZeroBoundedU16::<MAX_PULSER_RAW>::new_unchecked(val as u16)))
            }
        }
    }
}


// *,**AC Frequency, 20-400Hz, Res: 1Hz
pub struct AcFrequency(pub ClampedInclusiveU16<20, 400>);


// *,**,***AC Balance, 30-99%, Res: 1%
pub struct AcBalance(pub ClampedInclusiveU16<30, 99>);


// Preset Amps Min - PS Amps Max, Res: 1A
pub struct AcAmperageProfile(u16);


/*
HOT_START_TIME
    Range: 0(Off) -20
    Resolution: 0.1 Second
    Hot Start Enable / Disabled with Coil 8 Hot Start Enable.
    Stick only

HOT_START_EN
    *Hot Start Enable: 1 True / 0 False.
    Note: Hot Start can also be Disabled with 0 time set in Holding Register 6214 Hot Start Time.
    Stick only
*/
const MAX_HOT_START_RAW: u16 = 20;
pub struct HotStartProfile(pub NonZeroBoundedU16<MAX_HOT_START_RAW>);
impl HotStartProfile {
    pub fn as_seconds_option(&self) -> Option<f32> { self.0.0.map(|v| v.get() as f32 / 10.0) }
    pub fn try_from_f32(v: f32) -> Option<Self> {
        let val = (v * 10.0).round() as i32;
        match val{
            negative if negative < 0 => None,
            0 => Some(Self(NonZeroBoundedU16::ZERO_VALUE)),
            too_high if too_high > MAX_HOT_START_RAW as i32 => None,
            value => unsafe {
                // safety: We have checked the value is non-zero and within bounds
                Some(Self(NonZeroBoundedU16::<MAX_HOT_START_RAW>::new_unchecked(val as u16)))
            }
        }
    }
}


/*
AC EN Wave Shape,
    0: Advance Square,
    1: Soft Square,
    2: Sine,
    3: Triangle
*/
#[repr(u16)]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum WaveShape{
    AdvanceSquare = 0,
    SoftSquare = 1,
    Sine = 2,
    Triangle = 3,
}