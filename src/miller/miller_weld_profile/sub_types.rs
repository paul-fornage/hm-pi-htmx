use std::num::NonZeroU16;
use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use crate::error_targeted;



fn unwrap_opt_nz_u16(v: Option<NonZeroU16>) -> u16 { v.unwrap().get() }


pub trait AsModbusRegisterValue {
    fn as_modbus_register_value(&self) -> u16;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClampedInclusiveU16<const MIN: u16, const MAX: u16>(u16);
impl<const MIN: u16, const MAX: u16> ClampedInclusiveU16<MIN, MAX> {
    pub fn try_from_u16(val: u16) -> Option<Self> {
        if val > MAX { None } else if val < MIN { None } else { Some(Self(val)) }
    }
    pub fn value(&self) -> u16 { self.0 }
}



// *Hot Wire Voltage, 5-20, Res: 1V
pub type HotWireVoltage = ClampedInclusiveU16<5, 20>;


// *Postflow Time, 0(Off) - 50S & Auto(51), Res: 1Sec
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostFlowTime {
    Off,
    Manual(u8),
    Auto,
}
impl PostFlowTime {
    pub const OFF_VAL: u16 = 0;
    pub const MIN_MANUAL_VAL: u16 = Self::OFF_VAL + 1;
    pub const MAX_MANUAL_VAL: u16 = Self::AUTO_VAL - 1;
    pub const AUTO_VAL: u16 = 51;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NonZeroBoundedU16<const MAX: u16>(pub Option<NonZeroU16>);
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScaledNonZeroU16<const MAX: u16, const SCALE: u16>(pub NonZeroBoundedU16<MAX>);

impl<const MAX: u16, const SCALE: u16> ScaledNonZeroU16<MAX, SCALE> {
    pub const ZERO_VALUE: Self = Self(NonZeroBoundedU16::ZERO_VALUE);

    pub fn try_from_u16(val: u16) -> Option<Self> {
        NonZeroBoundedU16::try_from_u16(val).map(Self)
    }

    pub fn as_semantic_float(&self) -> Option<f32> {
        self.0.0.map(|v| v.get() as f32 / SCALE as f32)
    }

    pub fn from_semantic_float(v: f32) -> Option<Self> {
        let val = (v * SCALE as f32).round() as i32;
        match val {
            negative if negative < 0 => None,
            0 => Some(Self::ZERO_VALUE),
            too_high if too_high > MAX as i32 => None,
            val => unsafe {
                // safety: We have checked the value is non-zero and within bounds
                Some(Self(NonZeroBoundedU16::<MAX>::new_unchecked(val as u16)))
            }
        }
    }
}
impl<const MAX: u16, const SCALE: u16> AsModbusRegisterValue for ScaledNonZeroU16<MAX, SCALE> {
    fn as_modbus_register_value(&self) -> u16 {
        self.0.as_modbus_register_value()
    }
}


// from 0 (off) to MAX in tenths of a second
pub type WavePhaseDuration<const MAX: u16> = ScaledNonZeroU16<MAX, 10>;


// *Pulser - Background Amperage, 5-95%, Res: 1%
pub type PulserBackgroundAmps = ClampedInclusiveU16<5, 95>;


// *Pulser - Peak Time, 5-95%, Res: 1%
pub type PulserPeakTime = ClampedInclusiveU16<5, 95>;
impl AsModbusRegisterValue for ClampedInclusiveU16<5, 95> {
    fn as_modbus_register_value(&self) -> u16 { self.value() }
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
pub type PulseFrequency = ScaledNonZeroU16<MAX_PULSER_RAW, 10>;


// *,**AC Frequency, 20-400Hz, Res: 1Hz
pub type AcFrequency = ClampedInclusiveU16<20, 400>;


// *,**,***AC Balance, 30-99%, Res: 1%
pub type AcBalance = ClampedInclusiveU16<30, 99>;


// Preset Amps Min - PS Amps Max, Res: 1A
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcAmperageProfile(pub u16);


#[derive(Copy, Clone, FromPrimitive, IntoPrimitive, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum ElectrodePolarity {
    #[num_enum(default)]
    ElectrodeNegative = 0,
    ElectrodePositive = 1,
}


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
pub type HotStartProfile = ScaledNonZeroU16<MAX_HOT_START_RAW, 10>;


/*
ARC_START_TIME
    Arc Start Time: 0(Off) - 25(x10ms), Res: 1(x10ms)Write Only With Tungsten General
ARC_START_SLOPE_TIME
    Arc Start Slope Time: 0(Off) - 25(x10ms), Res: 1(x10ms)Write Only With Tungsten General
ARC_START_AC_TIME
    **Arc Start AC Time: 0(Off) - 25(x10ms), Res: 1(x10ms)Write Only With AC Power Source’s AC
    Output And Tungsten General
*/

pub const MAX_ARC_START_TIMING_RAW: u16 = 25;

pub type ArcStartTiming = ScaledNonZeroU16<MAX_ARC_START_TIMING_RAW, 100>;


/*
AC EN Wave Shape,
    0: Advance Square,
    1: Soft Square,
    2: Sine,
    3: Triangle
*/
#[repr(u16)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum WaveShape{
    AdvanceSquare = 0,
    SoftSquare = 1,
    Sine = 2,
    Triangle = 3,
}


/*
TUNGSTEN_PRESET
0 0.020 in. (0.5mm)
1 0.040 in. (1.0mm)
2 1/16 in. (1.6mm)
3 3/32 in. (2.4mm)
4 1/8 in. (3.2mm)
5 5/32 in. (4.0mm)
6 3/16 in. (4.8mm)
7 1/4 in. (6.4mm)
8 General (User Defined With Holding Registers 6207 Through 6212)
<9 Power Source Dependent, Typically Used With Process TIG
9 Disabled (Typically Used With Non TIG Processes)
*/
#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
pub enum TungstenPresetTag {
    Diameter0_5mm = 0,
    Diameter1_0mm = 1,
    Diameter1_6mm = 2,
    Diameter2_4mm = 3,
    Diameter3_2mm = 4,
    Diameter4_0mm = 5,
    Diameter4_8mm = 6,
    Diameter6_4mm = 7,
    #[num_enum(default)]
    General = 8,
    Disabled = 9,
}