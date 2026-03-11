use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::{Display, IntoStaticStr, VariantArray};

// *Postflow Time, 0(Off) - 50S & Auto(51), Res: 1Sec
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostFlowTime(u16);
impl PostFlowTime {
    pub const OFF_VAL: u16 = 0;
    pub const AUTO_VAL: u16 = 51;

    pub fn from_raw(raw: u16) -> Result<Self, String> {
        if raw <= 50 || raw == 51 {
            Ok(Self(raw))
        } else {
            Err(format!("Invalid postflow time: {}. Must be 0-50 or 51 (Auto)", raw))
        }
    }

    pub fn to_raw(self) -> u16 {
        self.0
    }

    pub fn display_value(self) -> String {
        if self.0 == Self::AUTO_VAL {
            "Auto".to_string()
        } else if self.0 == Self::OFF_VAL {
            "Off".to_string()
        } else {
            format!("{}s", self.0)
        }
    }
}


#[repr(u16)]
#[derive(Copy, Clone, Debug, Eq, PartialEq,
    TryFromPrimitive, IntoPrimitive,
    VariantArray, IntoStaticStr, Display)]
pub enum ElectrodePolarity {
    #[strum(to_string = "Electrode Neg.")]
    ElectrodeNegative = 0,
    #[strum(to_string = "Electrode Pos.")]
    ElectrodePositive = 1,
}


#[repr(u16)]
#[derive(Copy, Clone, Debug, Eq, PartialEq,
    TryFromPrimitive, IntoPrimitive,
    VariantArray, IntoStaticStr, Display)]
pub enum WaveShape {
    #[strum(to_string = "Advanced Square")]
    AdvancedSquare = 0,
    #[strum(to_string = "Soft Square")]
    SoftSquare = 1,
    #[strum(to_string = "Sine")]
    Sine = 2,
    #[strum(to_string = "Triangle")]
    Triangle = 3,
}


#[repr(u16)]
#[derive(Copy, Clone, Debug, Eq, PartialEq,
    TryFromPrimitive, IntoPrimitive,
    VariantArray, IntoStaticStr, Display)]
pub enum TungstenPreset {
    #[strum(to_string = "0.020\" (0.5mm)")]
    Diameter0_5mm = 0,
    #[strum(to_string = "0.040\" (1.0mm)")]
    Diameter1_0mm = 1,
    #[strum(to_string = "1/16\" (1.6mm)")]
    Diameter1_6mm = 2,
    #[strum(to_string = "3/32\" (2.4mm)")]
    Diameter2_4mm = 3,
    #[strum(to_string = "1/8\" (3.2mm)")]
    Diameter3_2mm = 4,
    #[strum(to_string = "5/32\" (4.0mm)")]
    Diameter4_0mm = 5,
    #[strum(to_string = "3/16\" (4.8mm)")]
    Diameter4_8mm = 6,
    #[strum(to_string = "1/4\" (6.4mm)")]
    Diameter6_4mm = 7,
    #[strum(to_string = "General (custom)")]
    General = 8,
    #[strum(to_string = "Disabled")]
    Disabled = 9,
}

