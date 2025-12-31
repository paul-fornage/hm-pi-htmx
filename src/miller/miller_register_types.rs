
use std::fmt::{Debug, Display};
use num_enum::{FromPrimitive, IntoPrimitive};
use std::marker::PhantomData;
pub use super::miller_error_registers::*;



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum WelderModel{
    Dynasty210 = 0,
    Dynasty280 = 1,
    Dynasty400 = 2,
    Dynasty800 = 3,
    Maxstar210 = 4,
    Maxstar280 = 5,
    Maxstar400 = 6,
    Maxstar800 = 7,
    Syncrowave300 = 8,
    Syncrowave400 = 9,
}

impl Display for WelderModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            WelderModel::Dynasty210 => "Dynasty 210",
            WelderModel::Dynasty280 => "Dynasty 280",
            WelderModel::Dynasty400 => "Dynasty 400",
            WelderModel::Dynasty800 => "Dynasty 800",
            WelderModel::Maxstar210 => "Maxstar 210",
            WelderModel::Maxstar280 => "Maxstar 280",
            WelderModel::Maxstar400 => "Maxstar 400",
            WelderModel::Maxstar800 => "Maxstar 800",
            WelderModel::Syncrowave300 => "Syncrowave 300",
            WelderModel::Syncrowave400 => "Syncrowave 400",
        };
        write!(f, "{}", display_str)
    }
}





#[repr(u16)]
#[derive(Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
pub enum WeldProcess {
    Stick = 0,
    Tig = 1,
    Mig = 2, // (Selectable only with Dynasty/Maxstar 210/280 Models and Dynasty’s Polarity DC)
    Test = 3,
    HotWire = 4,
    #[num_enum(default)]
    Unknown = 5,
}
impl Display for WeldProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_repr: &'static str = match self {
            WeldProcess::Stick => {"Stick"}
            WeldProcess::Tig => {"TIG"}
            WeldProcess::Mig => {"MIG"}
            WeldProcess::Test => {"Test"}
            WeldProcess::HotWire => {"Hot wire"}
            WeldProcess::Unknown => {"Unknown"}
        };
        write!(f, "{}", string_repr)
    }
}


#[repr(u16)]
#[derive(Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
pub enum WeldState{             // State:
    InitialAmperage = 0,    //  0 Initial Amperage
    InitialSlopeTime = 1,   //  1 Initial Slope Time
    MainAmperage = 2,       //  2 Main Amperage
    FinalSlopeTime = 3,     //  3 Final Slope Time
    FinalAmperage = 4,      //  4 Final Amperage
    Preflow = 5,            //  5 Preflow
    Standby = 6,            //  6 Standby
    OutputShorted = 7,      //  7 Output Shorted
    ReleaseTrigger = 8,     //  8 Release Trigger
    OutputDisabled = 9,     //  9 Output Disabled
    Error = 13,             //  13 Error
    PowerDown = 14,         //  14 Power Down
    PowerUp = 15,           //  15 Power Up
    #[num_enum(default)]
    Unknown = 16,
}
impl Display for WeldState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_repr: &'static str = match self {
            WeldState::InitialAmperage => {"initial amperage"}
            WeldState::InitialSlopeTime => {"initial slope time"}
            WeldState::MainAmperage => {"main amperage"}
            WeldState::FinalSlopeTime => {"final slope time"}
            WeldState::FinalAmperage => {"final amperage"}
            WeldState::Preflow => {"pre-flow"}
            WeldState::Standby => {"standby"}
            WeldState::OutputShorted => {"output shorted"}
            WeldState::ReleaseTrigger => {"release trigger"}
            WeldState::OutputDisabled => {"output disabled"}
            WeldState::Error => {"error"}
            WeldState::PowerDown => {"power down"}
            WeldState::PowerUp => {"power up"}
            WeldState::Unknown => {"unknown"}
        };
        write!(f, "{}", string_repr)
    }
}








/// Maps raw 5-bit value to character.
fn map_char_helper(val: u32) -> char {
    match val {
        0 => '@',
        1..=26 => char::from_u32(64 + val).unwrap(),
        _ => '?',
    }
}

/*

Application Software Number And Revision,
4 Bytes Bit Mapped:
NNNN,NNNN NNNN,NNNN NNNN,NNRR RRRE,EEEE

NNNN,NNNN NNNN,NNNN NNNN,NN == Miller Part Number,
    22 Bits 31 - 10, Bit Range 0 - 4,194,303, Actual 0-999999

RR RRR == Revision Level, 5 Bits 9 - 5, Bit Range 0 - 31,
    Actual 0 - 26
    where: 0 == “@” Preproduction Or Field Test Software
    1,2,3... == Revision A,B,C…
E,EEEE == Evaluation / Test, 5 Bits 9 - 5, Bit Range 0 - 31,
    Actual 0 - 26
    Where: 0 == ”@” Released Software,
    1,2,3... == Evaluation / Test Revision A,B,C…

*/

pub struct SubModuleSoftwareVersion(pub u32);
impl SubModuleSoftwareVersion {

    /// 22 Bits 31 - 10, Bit Range 0 - 4,194,303, Actual 0-999999
    pub fn get_part_number(&self) -> u32 {
        (self.0 >> 10) & ((1 << 22) - 1)
    }

    /// 5 Bits 9 - 5, Bit Range 0 - 31, Actual 0 - 26.
    /// where: 0 == “@” Preproduction Or Field Test Software
    /// 1,2,3... == Revision A,B,C…
    pub fn get_raw_revision_level(&self) -> u32 {
        (self.0 >> 5) & 0b11111
    }

    pub fn get_revision_level(&self) -> char {
        map_char_helper(self.get_raw_revision_level())
    }

    /// 5 bits 4 - 0, Bit Range 0 - 31, Actual 0 - 26.
    ///     Actual 0 - 26
    ///     Where: 0 == ”@” Released Software,
    ///     1,2,3... == Evaluation / Test Revision A,B,C…
    pub fn get_raw_evaluation_revision(&self) -> u32 {
        self.0 & 0b11111
    }

    pub fn get_evaluation_revision(&self) -> char {
        map_char_helper(self.get_raw_evaluation_revision())
    }
}


impl Debug for SubModuleSoftwareVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SubModuleSoftwareVersion: {{ \
            Part Number: {}, \
            Release Revision: {}, \
            Evaluation Revision: {} \
        }}",
               self.get_part_number(),
               self.get_revision_level(),
               self.get_evaluation_revision())
    }
}


impl Display for SubModuleSoftwareVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Part Number: {}, Release Revision: {}, Evaluation Revision: {}",
               self.get_part_number(),
               self.get_revision_level(),
               self.get_evaluation_revision())
    }
}



/*
Software Update Number Revision
Machine’s Software Update Number, Revision.
4 Bytes Bit Mapped:
NNNN,NNNN NNNN, NNNN NNNN,NNMM MMML,LLLL

NNNN,NNNN NNNN,NNNN NNNN,NN = Miller Part Number, 22 Bits 31−10,
    Bit Range 0−4,194,303, Actual 0−999999

MM MMM = Revision Level’s Most Significant Designator, 5 Bits 9−5, Bit
    Range 0−31, Actual 0,1−26 (ASCII “@,A−Z”), 9 “I” & 15 “O”
    Similar To “1” & “0” Not Used.
    Typically Starts At 0 (“@”, Omitted When Displayed), Increases By One With Each Wrap “Z” To “A”
    Of The Least Significant Designator

L, LLLL = Revision Level’s Least Significant Designator, 5 Bits 4−0, Bit
    Range 0−31, Actual 0,1−26 (ASCII “@,A−Z”), 9 “I” & 15 “O”
    Similar To “1” & “0” Not Used.
    0 “@” Used For Preproduction Only.

*/

pub struct SoftwareUpdateRevision(pub u32);
impl SoftwareUpdateRevision {
    /// 22 Bits 31 - 10, Bit Range 0 - 4,194,303, Actual 0-999999
    pub fn get_part_number(&self) -> u32 {
        (self.0 >> 10) & ((1 << 22) - 1)
    }

    /// 5 Bits 9 - 5
    /// MSD (Most Significant Designator)
    /// 0 ('@') is omitted when displayed.
    pub fn get_msd_raw(&self) -> u32 {
        (self.0 >> 5) & 0b11111
    }

    /// 5 Bits 4 - 0
    /// LSD (Least Significant Designator)
    pub fn get_lsd_raw(&self) -> u32 {
        self.0 & 0b11111
    }

    pub fn get_msd(&self) -> Option<char> {
        let val = self.get_msd_raw();
        if val == 0 {
            None
        } else {
            Some(map_char_helper(val))
        }
    }

    pub fn get_lsd(&self) -> char {
        map_char_helper(self.get_lsd_raw())
    }

    pub fn revision_str(&self) -> String {
        match self.get_msd() {
            Some(msd) => format!("{}{}", msd, self.get_lsd()),
            None => format!("{}", self.get_lsd())
        }
    }
}

impl Debug for SoftwareUpdateRevision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SoftwareUpdateRevision: {{ Part Number: {}, MSD: {:?}, LSD: {} }}",
               self.get_part_number(),
               self.get_msd_raw(), // Debug shows raw value for clarity
               self.get_lsd_raw()
        )
    }
}

impl Display for SoftwareUpdateRevision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Part Number: {}, Revision: {}",
               self.get_part_number(),
               self.revision_str())
    }
}

/*
Serial Number:
4 Bytes Bit Mapped:
DDDY,YYYW WWWW,WSSS SSSS,SSSS SSSB,BBBB

DDD = Decade Code, 3 Bits 31 - 29, Bit Range 0 - 7,
    actual “M” - “U” (For Decades 201*-208*), Skip “O”, See Note
Y,YYY = Year Code, 4 Bits 28 - 25, Bit Range 0 - 15, Actual 0 - 9
    “A” - “K”, Skip “I”, See Note
W WWWW,W = Week Number, 6 Bits 24-19, Bit Range 0 - 63,
    Actual 01 - 52
SSS SSSS,SSSS SSS = Serialized Number, 14 Bits 18 - 5,
    Bit Range 0 - 16383, Actual 0001-9999
B,BBBB = Business Unit Code, 5 Bits 4 - 0, Bit Range 0 - 31,
    Actual 0 - 25 “A”-”Z”, “I” And “O”, Not Used See Note

Note: Letters “I” And “O”, Similar To Numbers “1” And “0” Skipped In Decade And Year.
Not used In Business Unit Code.
 */


pub struct SerialNumber(pub u32);
impl SerialNumber {
    /// Decade Code, 3 Bits 31 - 29.
    /// actual “M” - “U” (For Decades 201*-208*), Skip “O”
    pub fn get_decade(&self) -> char {
        let val = (self.0 >> 29) & 0b111;
        // 'M' is 77. 'O' is 79.
        // 0 -> M, 1 -> N, 2 -> P (skipping O, which is +2 from M)
        let offset = if val >= 2 { 1 } else { 0 };
        char::from_u32('M' as u32 + val + offset).unwrap_or('?')
    }

    /// Year Code, 4 Bits 28 - 25.
    /// “A” - “K”, Skip “I”
    pub fn get_year(&self) -> char {
        let val = (self.0 >> 25) & 0b1111;
        // 'A' is 65. 'I' is 73 (index 8).
        // 0 -> A ... 7 -> H, 8 -> J (skipping I)
        let offset = if val >= 8 { 1 } else { 0 };
        char::from_u32('A' as u32 + val + offset).unwrap_or('?')
    }

    /// Week Number, 6 Bits 24-19
    pub fn get_week(&self) -> u32 {
        (self.0 >> 19) & 0b111111
    }

    /// Serialized Number, 14 Bits 18 - 5
    pub fn get_serialized_number(&self) -> u32 {
        (self.0 >> 5) & 0b11_1111_1111_1111
    }

    /// Business Unit Code, 5 Bits 4 - 0
    /// Actual 0 - 25 “A”-”Z”
    pub fn get_business_unit(&self) -> char {
        let val = self.0 & 0b11111;
        char::from_u32('A' as u32 + val).unwrap_or('?')
    }
}

impl Display for SerialNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{:02}{:04}{}",
               self.get_decade(),
               self.get_year(),
               self.get_week(),
               self.get_serialized_number(),
               self.get_business_unit())
    }
}

impl Debug for SerialNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Serial Number: {}", self)
    }
}