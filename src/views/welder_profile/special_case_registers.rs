use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

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
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum ElectrodePolarity {
    ElectrodeNegative = 0,
    ElectrodePositive = 1,
}
impl ElectrodePolarity {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::ElectrodeNegative => "EN (Electrode Negative)",
            Self::ElectrodePositive => "EP (Electrode Positive)",
        }
    }

    pub fn all_variants() -> &'static [(u16, &'static str)] {
        &[
            (0, "EN (Electrode Negative)"),
            (1, "EP (Electrode Positive)"),
        ]
    }
}


#[repr(u16)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum WaveShape {
    AdvancedSquare = 0,
    SoftSquare = 1,
    Sine = 2,
    Triangle = 3,
}
impl WaveShape {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::AdvancedSquare => "Advanced Square",
            Self::SoftSquare => "Soft Square",
            Self::Sine => "Sine",
            Self::Triangle => "Triangle",
        }
    }

    pub fn all_variants() -> &'static [(u16, &'static str)] {
        &[
            (0, "Advanced Square"),
            (1, "Soft Square"),
            (2, "Sine"),
            (3, "Triangle"),
        ]
    }
}


#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum TungstenPreset {
    Diameter0_5mm = 0,
    Diameter1_0mm = 1,
    Diameter1_6mm = 2,
    Diameter2_4mm = 3,
    Diameter3_2mm = 4,
    Diameter4_0mm = 5,
    Diameter4_8mm = 6,
    Diameter6_4mm = 7,
    General = 8,
    Disabled = 9,
}
impl TungstenPreset {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Diameter0_5mm => "0.020\" (0.5mm)",
            Self::Diameter1_0mm => "0.040\" (1.0mm)",
            Self::Diameter1_6mm => "1/16\" (1.6mm)",
            Self::Diameter2_4mm => "3/32\" (2.4mm)",
            Self::Diameter3_2mm => "1/8\" (3.2mm)",
            Self::Diameter4_0mm => "5/32\" (4.0mm)",
            Self::Diameter4_8mm => "3/16\" (4.8mm)",
            Self::Diameter6_4mm => "1/4\" (6.4mm)",
            Self::General => "General (User Defined)",
            Self::Disabled => "Disabled",
        }
    }

    pub fn all_variants() -> &'static [(u16, &'static str)] {
        &[
            (0, "0.020\" (0.5mm)"),
            (1, "0.040\" (1.0mm)"),
            (2, "1/16\" (1.6mm)"),
            (3, "3/32\" (2.4mm)"),
            (4, "1/8\" (3.2mm)"),
            (5, "5/32\" (4.0mm)"),
            (6, "3/16\" (4.8mm)"),
            (7, "1/4\" (6.4mm)"),
            (8, "General (User Defined)"),
            (9, "Disabled"),
        ]
    }
}
