mod sub_types;

use num_enum::FromPrimitive;
use sub_types::*;
use crate::error::Error;
use crate::miller::miller_memory::MillerMemory;
use crate::miller::miller_register_definitions::*;
use crate::modbus::RegisterAddress;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MillerWeldProfile {
    /*0004 AC Power Source’s Output DC: 1 True (DC) / 0 False (AC).*/
    pub use_dc_output: bool,
    /*0005 AC Power Source’s DC Polarity EP: 1 True (EP) / 0 False (EN).*/
    pub use_ep_polarity: bool,
    /*0008 Boost Enable: 1 True / 0 False.*/
    pub boost_en: bool,
    /*0009 Droop Enable: 1 True / 0 False.*/
    pub droop_en: bool,
    /*0010 Open Circuit Voltage (OCV) Low Enable: 1 True (Low) / 0 False (Normal). 
    OCV selection applies to both Stick and MIG processes.*/
    pub use_low_ocv: bool,
    /*0017 Pulser Enable: 1 True / 0 False. 
    Note: Can also be set TRUE / FALSE when writing values to Holding Register 6305 
    Pulser Pulses Per Second (PPS). When enabled and Holding Register 6304 PPS is found at “0”, 
    PPS will be set to a default value.*/
    pub pulser_en: bool,
    /*0018 Dynasty/Maxstar 400/800 Models Only *
    AC Commutation Amperage LOW ENABLE: 1 TRUE (LOW) / 0 FALSE (High) 
    Application: Use High commutation amperage when a more aggressive arc is preferred. 
    Use Low commutation amperage when a less aggressive and quieter arc is preferred.
    */
    pub use_low_ac_commutation_amp: bool,
    /*0019 AC Independent Enable: 1 True / 0 False. Enables/Disables Both Independent Amperage and 
    Independent AC Wave Shapes.*/
    pub ac_independent_en: bool,
    /*6205 Tungsten (Canned Arc Start Parameters):
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
    9 Disabled (Typically Used With Non TIG Processes)*/
    pub tungsten_preset: TungstenPresetTag,
    /*6206 Preset Amperage Minimum: Power Source AC / DC Amperage Minimum -25A(Tungsten General) Or 
    63A(Tungsten Disabled), Res 1A
    Write Only With Tungsten General Or Disabled*/
    pub preset_min_amperage: ClampedInclusiveU16<1, 63>,
    /*6207 Arc Start Amperage: 5A - 200A, Res: 1AWrite Only With Tungsten General Or Disabled*/
    pub arc_start_amperage: ClampedInclusiveU16<5, 200>,
    /*6208 Arc Start Time: 0(Off) - 25(x10ms), Res: 1(x10ms)Write Only With Tungsten General*/
    pub arc_start_time: ArcStartTiming,
    /*6209 Arc Start Slope Time: 0(Off) - 25(x10ms), Res: 1(x10ms)Write Only With Tungsten General*/
    pub arc_start_slope_time: ArcStartTiming,
    /*6210 Arc Start AC Time: 0(Off) - 25(x10ms), Res: 1(x10ms)Write Only With AC Power Source’s AC 
    Output And Tungsten General*/
    pub arc_start_ac_time: ArcStartTiming,
    /*6211 Arc Start Polarity Phase: 1 EP, 0 ENWrite Only With AC Power Source And Tungsten General 
    or Disabled*/
    pub arc_start_polarity_phase: ElectrodePolarity,
    /*6212 AC EN Wave Shape, 0 Advance Square, 1 Soft Square, 2 Sine, 3 Triangle*/
    pub ac_en_wave_shape: WaveShape,
    /*6213 AC EP Wave Shape, 0 Advance Square, 1 Soft Square, 2 Sine, 3 Triangle*/
    pub ac_ep_wave_shape: WaveShape,
    /*6214 Hot Start Time:
    Range: 0(Off) -20
    Resolution: 0.1 Second
    Hot Start Enable / Disabled with Coil 8 Hot Start Enable.
    Stick only*/
    pub hot_start_time: HotStartProfile,
    /*6300 AC EN Amperage, Preset Amps Min - PS Amps Max, Res: 1A*/
    pub ac_en_amperage: AcAmperageProfile,
    /*6301 AC EP Amperage, Preset Amps Min - PS Amps Max, Res: 1A*/
    pub ac_ep_amperage: AcAmperageProfile,
    /*6302 AC Balance, 30-99%, Res: 1%*/
    pub ac_balance: AcBalance,
    /*6303 AC Frequency, 20-400Hz, Res: 1Hz*/
    pub ac_frequency: AcFrequency,
    /*6304 Weld Amperage(DC or AC), Preset Amps Min - PS Amps Max, Res: 1A*/
    pub weld_amperage: AcAmperageProfile,
    /*6305 Pulser - Pulses Per Second (PPS)
    Range: 0(Off) – 50000 / 5000 Power Source Dependent,
    Resolution: 0.1 Hertz
    Can be set to a default value when writing a TRUE to coil 18 Pulser Enable and PPS is found at 0(Off).
    Writing a non “0” value will set coil 18 Pulser Enable to TRUE.
    Writing a “0” value will set coil 18 Pulser Enable to FALSE.
    Dependent on configuration of the slave, the slave may or may not retain the PPS non “0” value.*/
    pub pulser_pps: PulseFrequency,
    /*6306 Pulser - Peak Time, 5-95%, Res: 1%*/
    pub pulser_peak_time: PulserPeakTime,
    /*6308 Prelow Time, 0(Off) - 250, Res: 1(x0.1Sec)*/
    pub preflow_time: WavePhaseDuration<250>,
    /*6309 Initial Amperage, Preset Amps Min - PS Amps Max, Res: 1A*/
    pub initial_amperage: AcAmperageProfile,
    /*6310 Initial Time, 0(Off) - 250, Res: 1(x0.1Sec)*/
    pub initial_time: WavePhaseDuration<250>,
    /*6311 Initial Slope Time, 0(Off) - 500, Res: 1(x0.1Sec)*/
    pub initial_slope_time: WavePhaseDuration<500>,
    /*6312 Main Time, 0(Off) - 9990, Res: 1(x0.1Sec)*/
    pub main_time: WavePhaseDuration<9990>,
    /*6313 Final Slope Time, 0(Off) - 500, Res: 1(x0.1Sec)*/
    pub final_slope_time: WavePhaseDuration<500>,
    /*6314 Final Amperage, Preset Amps Min - PS Amps Max, Res: 1A*/
    pub final_amperage: AcAmperageProfile,
    /*6315 Final Time, 0(Off) - 250, Res: 1(x0.1Sec)*/
    pub final_time: WavePhaseDuration<250>,
    /*6316 Postflow Time, 0(Off) - 50S & Auto(51), Res: 1Sec*/
    pub postflow_time: PostFlowTime,
    /*6318 Hot Wire Voltage, 5-20, Res: 1V*/
    pub hot_wire_voltage: HotWireVoltage,
}

async fn read_coil_result(
    memory: &MillerMemory,
    address: RegisterAddress,
) -> Result<bool, Error> {
    memory
        .read_coil(address.address)
        .await
        .ok_or(Error::ReadUnpopulatedRegister(address))
}
async fn read_hreg_result(
    memory: &MillerMemory,
    address: RegisterAddress,
) -> Result<u16, Error> {
    memory
        .read_hreg(address.address)
        .await
        .ok_or(Error::ReadUnpopulatedRegister(address))
}

impl MillerWeldProfile {
    pub async fn pull_from_mb(memory: &MillerMemory) -> Result<Self, Error> {
        let use_dc_output = read_coil_result(memory, USE_DC_OUTPUT.address).await?;
        let use_ep_polarity = read_coil_result(memory, USE_EP_POLARITY.address).await?;
        let boost_en = read_coil_result(memory, BOOST_EN.address).await?;
        let droop_en = read_coil_result(memory, DROOP_EN.address).await?;
        let use_low_ocv = read_coil_result(memory, USE_LOW_OCV.address).await?;
        let pulser_en = read_coil_result(memory, PULSER_EN.address).await?;
        let use_low_ac_commutation_amp =
            read_coil_result(memory, USE_LOW_AC_COMMUTATION_AMP.address).await?;
        let ac_independent_en = read_coil_result(memory, AC_INDEPENDANT_EN.address).await?;

        let tungsten_preset =
            read_hreg_result(memory, TUNGSTEN_PRESET.address).await?.into();

        let preset_min_amperage = ClampedInclusiveU16::try_from_u16(
            read_hreg_result(memory, PRESET_MIN_AMPERAGE.address).await?,
        )
            .ok_or(Error::ReadUnpopulatedRegister(PRESET_MIN_AMPERAGE.address))?;

        let arc_start_amperage = ClampedInclusiveU16::try_from_u16(
            read_hreg_result(memory, ARC_START_AMPERAGE.address).await?,
        )
            .ok_or(Error::ReadUnpopulatedRegister(ARC_START_AMPERAGE.address))?;

        let arc_start_time = ArcStartTiming::try_from_u16(
            read_hreg_result(memory, ARC_START_TIME.address).await?,
        )
            .ok_or(Error::ReadUnpopulatedRegister(ARC_START_TIME.address))?;

        let arc_start_slope_time = ArcStartTiming::try_from_u16(
            read_hreg_result(memory, ARC_START_SLOPE_TIME.address).await?,
        )
            .ok_or(Error::ReadUnpopulatedRegister(ARC_START_SLOPE_TIME.address))?;

        let arc_start_ac_time = ArcStartTiming::try_from_u16(
            read_hreg_result(memory, ARC_START_AC_TIME.address).await?,
        )
            .ok_or(Error::ReadUnpopulatedRegister(ARC_START_AC_TIME.address))?;

        let arc_start_polarity_phase = ElectrodePolarity::from_primitive(
            read_hreg_result(memory, ARC_START_POLARITY_PHASE.address).await?,
        );

        let ac_en_wave_shape = WaveShape::try_from(
            read_hreg_result(memory, AC_EN_WAVE_SHAPE.address).await?,
        )
            .map_err(|_| Error::ReadUnpopulatedRegister(AC_EN_WAVE_SHAPE.address))?;

        let ac_ep_wave_shape = WaveShape::try_from(
            read_hreg_result(memory, AC_EP_WAVE_SHAPE.address).await?,
        )
            .map_err(|_| Error::ReadUnpopulatedRegister(AC_EP_WAVE_SHAPE.address))?;

        let hot_start_time = HotStartProfile::try_from_u16(
            read_hreg_result(memory, HOT_START_TIME.address).await?,
        )
            .ok_or(Error::ReadUnpopulatedRegister(HOT_START_TIME.address))?;

        let ac_en_amperage = AcAmperageProfile(
            read_hreg_result(memory, AC_EN_AMPERAGE.address).await?,
        );
        let ac_ep_amperage = AcAmperageProfile(
            read_hreg_result(memory, AC_EP_AMPERAGE.address).await?,
        );

        let ac_balance = AcBalance::try_from_u16(
                read_hreg_result(memory, AC_BALANCE.address).await?,
            ).ok_or(Error::ReadUnpopulatedRegister(AC_BALANCE.address))?;

        let ac_frequency = AcFrequency::try_from_u16(
                read_hreg_result(memory, AC_FREQUENCY.address).await?,
            ).ok_or(Error::ReadUnpopulatedRegister(AC_FREQUENCY.address))?;

        let weld_amperage = AcAmperageProfile(
            read_hreg_result(memory, WELD_AMPERAGE.address).await?,
        );

        let pulser_pps = PulseFrequency::try_from_u16(
            read_hreg_result(memory, PULSER_PPS.address).await?,
        )
            .ok_or(Error::ReadUnpopulatedRegister(PULSER_PPS.address))?;

        let pulser_peak_time = PulserPeakTime::try_from_u16(
                read_hreg_result(memory, PULSER_PEAK_TIME.address).await?,
            ).ok_or(Error::ReadUnpopulatedRegister(PULSER_PEAK_TIME.address))?;

        let preflow_time = WavePhaseDuration::try_from_u16(
            read_hreg_result(memory, PREFLOW_TIME.address).await?,
        ).ok_or(Error::ReadUnpopulatedRegister(PREFLOW_TIME.address))?;

        let initial_amperage = AcAmperageProfile(
            read_hreg_result(memory, INITIAL_AMPERAGE.address).await?,
        );

        let initial_time = WavePhaseDuration::try_from_u16(
            read_hreg_result(memory, INITIAL_TIME.address).await?,
        ).ok_or(Error::ReadUnpopulatedRegister(INITIAL_TIME.address))?;

        let initial_slope_time = WavePhaseDuration::try_from_u16(
            read_hreg_result(memory, INITIAL_SLOPE_TIME.address).await?,
        ).ok_or(Error::ReadUnpopulatedRegister(INITIAL_SLOPE_TIME.address))?;

        let main_time = WavePhaseDuration::try_from_u16(
            read_hreg_result(memory, MAIN_TIME.address).await?,
        ).ok_or(Error::ReadUnpopulatedRegister(MAIN_TIME.address))?;

        let final_slope_time = WavePhaseDuration::try_from_u16(
            read_hreg_result(memory, FINAL_SLOPE_TIME.address).await?,
        ).ok_or(Error::ReadUnpopulatedRegister(FINAL_SLOPE_TIME.address))?;

        let final_amperage = AcAmperageProfile(
            read_hreg_result(memory, FINAL_AMPERAGE.address).await?,
        );

        let final_time = WavePhaseDuration::try_from_u16(
            read_hreg_result(memory, FINAL_TIME.address).await?,
        ).ok_or(Error::ReadUnpopulatedRegister(FINAL_TIME.address))?;

        let postflow_time = match read_hreg_result(memory, POSTFLOW_TIME.address).await?
        {
            PostFlowTime::OFF_VAL => PostFlowTime::Off,
            PostFlowTime::AUTO_VAL => PostFlowTime::Auto,
            val => PostFlowTime::Manual(val as u8),
        };

        let hot_wire_voltage = HotWireVoltage::try_from_u16(
                read_hreg_result(memory, HOT_WIRE_VOLTAGE.address).await?,
            ).ok_or(Error::ReadUnpopulatedRegister(HOT_WIRE_VOLTAGE.address))?;

        Ok(Self {
            use_dc_output,
            use_ep_polarity,
            boost_en,
            droop_en,
            use_low_ocv,
            pulser_en,
            use_low_ac_commutation_amp,
            ac_independent_en,
            tungsten_preset,
            preset_min_amperage,
            arc_start_amperage,
            arc_start_time,
            arc_start_slope_time,
            arc_start_ac_time,
            arc_start_polarity_phase,
            ac_en_wave_shape,
            ac_ep_wave_shape,
            hot_start_time,
            ac_en_amperage,
            ac_ep_amperage,
            ac_balance,
            ac_frequency,
            weld_amperage,
            pulser_pps,
            pulser_peak_time,
            preflow_time,
            initial_amperage,
            initial_time,
            initial_slope_time,
            main_time,
            final_slope_time,
            final_amperage,
            final_time,
            postflow_time,
            hot_wire_voltage,
        })
    }
}



