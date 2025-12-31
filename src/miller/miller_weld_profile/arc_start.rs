use num_enum::{IntoPrimitive, TryFromPrimitive};

pub enum ArcStartProfile {
    TungstenPreset(TungstenPresetTagSize),
    Manual(ManualArcStartProfile),
    Disabled(ArcStartAmperageProfile),
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
#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum TungstenPresetTagSize {
    Diameter0_5mm = 0,
    Diameter1_0mm = 1,
    Diameter1_6mm = 2,
    Diameter2_4mm = 3,
    Diameter3_2mm = 4,
    Diameter4_0mm = 5,
    Diameter4_8mm = 6,
    Diameter6_4mm = 7,
}


/*
PRESET_MIN_AMPERAGE
    Preset Amperage Minimum:
    range: {Power Source AC / DC Amperage Minimum} .. {25A}(when Tungsten General) Or
    {63A}(when Tungsten Disabled), Res 1A Write Only With Tungsten General Or Disabled
ARC_START_AMPERAGE
    Arc Start Amperage: 5A - 200A, Res: 1AWrite Only With Tungsten General Or Disabled
ARC_START_TIME
    Arc Start Time: 0(Off) - 25(x10ms), Res: 1(x10ms)Write Only With Tungsten General
ARC_START_SLOPE_TIME
    Arc Start Slope Time: 0(Off) - 25(x10ms), Res: 1(x10ms)Write Only With Tungsten General
ARC_START_AC_TIME
    **Arc Start AC Time: 0(Off) - 25(x10ms), Res: 1(x10ms)Write Only With AC Power Source’s AC
    Output And Tungsten General
ARC_START_POLARITY_PHASE
    **Arc Start Polarity Phase: 1 EP, 0 ENWrite Only With AC Power Source And Tungsten General or
    Disabled
 */
pub struct ManualArcStartProfile {
    pub amperage_profile: ArcStartAmperageProfile,
    pub arc_start_time: u16,
    pub arc_start_slope_time: u16,
    pub arc_start_ac_time: u16,
    pub arc_start_polarity_phase_use_ep: bool,
}

pub struct ArcStartAmperageProfile {
    pub min_amperage: u16,
    pub arc_start_amperage: u16,
}
