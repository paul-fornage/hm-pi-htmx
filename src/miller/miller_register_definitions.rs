use crate::modbus::{ModbusAddressType, RegisterAddress, RegisterMetadata};
use crate::modbus::cached_modbus::ModbusChunk;


pub const MILLER_CHUNKS: &'static[ModbusChunk] = &[
    ModbusChunk::Coils{address: 0, count: 21},
    ModbusChunk::DiscreteInputs{address: 2000, count: 19},
    ModbusChunk::InputRegisters{address: 4016, count: 22},
    ModbusChunk::InputRegisters{address: 4099, count: 5},
    ModbusChunk::InputRegisters{address: 4200, count: 7},
    ModbusChunk::InputRegisters{address: 4300, count: 8},
    ModbusChunk::InputRegisters{address: 4400, count: 9},
    ModbusChunk::HoldingRegisters{address: 6000, count: 4},
    ModbusChunk::HoldingRegisters{address: 6100, count: 4},
    ModbusChunk::HoldingRegisters{address: 6200, count: 18},
    ModbusChunk::HoldingRegisters{address: 6300, count: 19},
];


/**
 * User Interface Disable: 1 True / 0 False.
 * With User Interface disabled, all "*" marked Coils and Holding Registers should be set for
 * desired function.
 *
 * Context: Disable the front panel UI so Modbus can control starred settings without the UI
 * overwriting them.
 */
pub const PS_UI_DISABLE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0000 },
    name: "PS UI DISABLE",
    description: "Disables the front-panel UI so Modbus can control starred settings without the panel overriding them. Re-enable to return local control.",
};

/**
 * Rmt Trigger Disable: 1 True / 0 False.
 *
 * Context: Blocks the remote trigger input on the 14-pin connector so external start/stop is
 * ignored.
 */
pub const RMT_TRIGGER_DISABLE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0001 },
    name: "RMT TRIGGER DISABLE",
    description: "Disables the remote trigger input on the 14-pin connector.",
};

/**
 * Trigger (Contactor) Request: 1 True (1 Second Time Out Return To False) / 0 False.
 * To continue a weld sequence through Final Slope and or Final Time, Coil must be refreshed with
 * False throughout these sequences.
 *
 * Context: Momentary request to close the contactor (start output). Used to start or continue a
 * weld sequence.
 */
pub const CONTACTOR_TRIGGER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0002 },
    name: "CONTACTOR TRIGGER",
    description: "Momentary request to close the contactor (start output).",
};

/**
 * Gas Request: 1 True (1 Second Time Out Return To False) / 0 False.
 *
 * Context: Momentary request for gas flow independent of weld output.
 */
pub const GAS_REQUEST: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0003 },
    name: "GAS REQUEST",
    description: "Momentary request for gas flow independent of weld output.",
};

/**
 * * AC Power Source's Output DC: 1 True (DC) / 0 False (AC).
 *
 * Context: Selects DC output instead of AC on AC-capable machines.
 */
pub const USE_DC_OUTPUT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0004 },
    name: "USE DC OUTPUT",
    description: "Selects DC output instead of AC (AC-capable models only).",
};

/**
 * * AC Power Source's DC Polarity EP: 1 True (EP) / 0 False (EN).
 *
 * Context: Selects electrode-positive (EP) polarity for DC output.
 */
pub const USE_EP_POLARITY: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0005 },
    name: "USE EP POLARITY",
    description: "Selects electrode-positive (EP) polarity for DC output.",
};

/**
 * * Stuck Check Enable: 1 True / 0 False.
 * When Stick Stuck Check is on and the welding electrode (rod) is stuck, output is turned off.
 */
pub const STUCK_CHECK_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0006 },
    name: "STUCK CHECK EN",
    description: "Enables stick-stuck protection that shuts off output if the electrode sticks.",
};

/**
 * * Hot Start Enable: 1 True / 0 False.
 * Note: Hot Start can also be disabled with 0 time set in Holding Register 6215 Hot Start Time.
 * Stick only.
 *
 * Context: Project uses holding register 6214 for hot start time.
 */
pub const HOT_START_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0007 },
    name: "HOT START EN",
    description: "Enables hot start boost at arc start for stick welding.",
};

/**
 * * Boost Enable: 1 True / 0 False.
 */
pub const BOOST_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0008 },
    name: "BOOST EN",
    description: "Enables boost (arc force) to increase current during short arc conditions.",
};

/**
 * * Droop Enable: 1 True / 0 False.
 */
pub const DROOP_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0009 },
    name: "DROOP EN",
    description: "Enables droop mode to soften the arc by reducing voltage as current rises.",
};

/**
 * * Open Circuit Voltage (OCV) Low Enable: 1 True (Low) / 0 False (Normal).
 * OCV selection applies to both Stick and MIG processes.
 */
pub const USE_LOW_OCV: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0010 },
    name: "USE LOW OCV",
    description: "Uses low open-circuit voltage (applies to stick and MIG).",
};

/**
 * * Weld Gas Enable: 1 True / 0 False Enables Gas With Contactor.
 *
 * Context: Set true for automatic gas, false if using an external gas solenoid.
 */
pub const GAS_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0011 },
    name: "GAS EN",
    description: "Enables automatic gas valve operation with the contactor.",
};

/**
 * Non-CE Models Only Cooler Power Supply (CPS) Enable: 1 True (Parallel With Coil 0013) / 0 False.
 * Note: Dynasty/Maxstar 210/280 CE Models have no control; read returns False.
 *
 * Context: Enables the 120 V cooler power supply output.
 */
pub const COOLER_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0012 },
    name: "COOLER EN",
    description: "Enables the cooler power supply output (non-CE models only).",
};

/**
 * * Cooler Power Supply (CPS) TIG Enable: 1 True (Parallel With Coil 0012) / 0 False.
 * TIG Process Control of Cooler Power Supply.
 */
pub const COOLER_TIG_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0013 },
    name: "COOLER TIG EN",
    description: "Allows TIG process control of the cooler power supply.",
};

/**
 * Dynasty/Maxstar 210/280 Models Only * Cooler Error Enable: 1 True / 0 False.
 * Enables Error "1.3.6 No Cooler Detected With Output Current".
 * Error is generated when no load detected on cooler power supply output with load detected on the
 * power source output.
 */
pub const COOLER_ERROR_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0014 },
    name: "COOLER ERROR EN",
    description: "Enables the 'no cooler detected with output current' fault.",
};

/**
 * Touch Sense Enable: 1 True / 0 False.
 * Touch Sense Detection found at Modbus Discrete Input 2009 or Remote 14 Receptacle Socket J.
 *
 * Context: Touch sense detect status is exposed in this project at discrete input 2008
 * (TOUCH_SENSE_DETECT).
 */
pub const TOUCH_SENSE_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0015 },
    name: "TOUCH SENSE EN",
    description: "Enables touch sense, which detects the electrode touching the work before arc start.",
};

/**
 * RMS Enable: AC Amperage Preset And Meter And/Or DC Pulse Amperage Meter:
 * 1 True (RMS) / 0 False (Average).
 * Note: To enable, must have Discrete Input 2013 RMS Hardware Detect = True.
 *
 * Context: RMS hardware detect is exposed here as discrete input 2012 (RMS_HW_PRESENT).
 */
pub const RMS_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0016 },
    name: "RMS EN",
    description: "Uses RMS measurement mode for AC amperage and DC pulse metering (if hardware present).",
};

/**
 * * Pulser Enable: 1 True / 0 False.
 * Note: Can also be set TRUE / FALSE when writing values to Holding Register 6305 Pulser Pulses
 * Per Second (PPS). When enabled and Holding Register 6304 PPS is found at "0", PPS will be set to
 * a default value.
 *
 * Context: In this project, PPS is register 6305; 6304 is weld amperage.
 */
pub const PULSER_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0017 },
    name: "PULSER EN",
    description: "Enables pulsed output.",
};

/**
 * Dynasty/Maxstar 400/800 Models Only * AC Commutation Amperage Low Enable:
 * 1 True (Low) / 0 False (High).
 * Application: Use high commutation amperage when a more aggressive arc is preferred. Use low
 * commutation amperage when a less aggressive and quieter arc is preferred.
 */
pub const USE_LOW_AC_COMMUTATION_AMP: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0018 },
    name: "USE LOW AC COMMUTATION AMP",
    description: "Uses low AC commutation amperage for a softer, quieter arc (400/800 models).",
};

/**
 * * AC Independent Enable: 1 True / 0 False.
 * Enables/Disables both independent amperage and independent AC wave shapes.
 */
pub const AC_INDEPENDANT_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0019 },
    name: "AC INDEPENDANT EN",
    description: "Enables independent EN/EP amperage and wave shape controls for AC.",
};

/**
 * * Weld Timers Enable: 1 True / 0 False.
 * Weld Timers include weld (spot), initial amperage, and final amperage timers.
 */
pub const WLED_TIMERS_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0020 },
    name: "WLED TIMERS EN",
    description: "Enables weld timers (spot weld, initial, and final timers).",
};

/**
 * Dynasty/Maxstar 210/280 Models Only. Cooler Power Supply (CPS) Detect: 1 True / 0 False.
 */
pub const COOLER_DETECTED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2000 },
    name: "COOLER DETECTED",
    description: "Cooler power supply detected (210/280 only).",
};

/**
 * Dynasty/Maxstar 210/280 Models Only. Cooler Load Detect: 1 True / 0 False.
 */
pub const COOLER_LOAD_DETECTED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2001 },
    name: "COOLER LOAD DETECTED",
    description: "Cooler load detected (210/280 only). Indicates a cooler is connected and drawing load.",
};

/**
 * Foot/Finger Tip Control Detect: 1 True / 0 False.
 * Note: Holding Register 6205 (Remote 14-Skt E) Must Be Configured To 0 (Amperage Control) To
 * Detect Foot/Finger Tip Control.
 *
 * Context: This project uses holding register 6204 (RMT_PIN_E_CONFIG) for the Remote 14-skt E
 * configuration.
 */
pub const FOOT_CONTROL_DETECTED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2002 },
    name: "FOOT CONTROL DETECTED",
    description: "Foot or finger control detected on the Remote 14 input (requires pin E configured for amperage control).",
};

/**
 * Remote Trigger (Contactor 14-Skt A-B) Enable: 1 True / 0 False.
 */
pub const RMT_TRIGGER_ENABLED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2003 },
    name: "RMT TRIGGER ENABLED",
    description: "Remote trigger circuit enabled (contactor A-B on the Remote 14 connector).",
};

/**
 * Contactor Output Enabled: 1 True / 0 False (Contactor Output Or Sense Voltage Pre Contactor
 * Output).
 */
pub const CONTACTOR_OUTPUT_ENABLED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2004 },
    name: "CONTACTOR OUTPUT ENABLED",
    description: "Contactor output active (weld output enabled or sensing voltage pre-contactor).",
};

/**
 * Gas Output Enabled: 1 True / 0 False.
 */
pub const GAS_OUTPUT_ENABLED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2005 },
    name: "GAS OUTPUT ENABLED",
    description: "Gas valve output active.",
};

/**
 * Valid Arc: 1 True / 0 False.
 */
pub const IS_VALID_ARC: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2006 },
    name: "IS VALID ARC",
    description: "Arc detected as valid.",
};

/**
 * Arc Length Control Lock Out: 1 True / 0 False.
 */
pub const ARC_LENGTH_CTL_LOCKOUT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2007 },
    name: "ARC LENGTH CTL LOCKOUT",
    description: "Arc length control lockout active.",
};

/**
 * Touch Sense Detect: 1 True / 0 False.
 * Touch Sense Enable (Coil 16) must be set True with Machine's State (Input Register 4101) in
 * standby, and weld output shorted for Touch Sense Detect to register as True.
 *
 * Context: Touch sense enable is coil 0015. Weld state is input register 4100 in this project.
 */
pub const TOUCH_SENSE_DETECT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2008 },
    name: "TOUCH SENSE DETECT",
    description: "Touch sense detected (electrode touching the work with output shorted in standby).",
};

/**
 * CE Model Detect: 1 True / 0 False.
 */
pub const IS_CE_MODEL: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2009 },
    name: "IS CE MODEL",
    description: "CE model detected.",
};

/**
 * STR Model Detect: 1 True / 0 False.
 */
pub const IS_STR_MODEL: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2010 },
    name: "IS STR MODEL",
    description: "STR model detected.",
};

/**
 * DX Model Detect: 1 True / 0 False.
 */
pub const IS_DX_MODEL: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2011 },
    name: "IS DX MODEL",
    description: "DX model detected.",
};

/**
 * RMS Hardware Detect: 1 True / 0 False.
 */
pub const RMS_HW_PRESENT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2012 },
    name: "RMS HW PRESENT",
    description: "RMS measurement hardware present.",
};

/**
 * Low Line Detect: 1 True / 0 False (Dynasty/Maxstar 210 Only).
 * Note: Set True when powered up on 120 V input.
 */
pub const LOW_LIVE_INPUT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2013 },
    name: "LOW LIVE INPUT",
    description: "Low-line (120 V) input detected (210 only).",
};

/**
 * Feature Enable for Hot Start Adjust: 1 True / 0 False.
 */
pub const HOT_START_SUPPORTED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2014 },
    name: "HOT START SUPPORTED",
    description: "Hot start adjustment supported by this machine.",
};

/**
 * Feature Enable for AC Independent: 1 True / 0 False.
 */
pub const AC_INDEPENDANT_SUPPORTED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2015 },
    name: "AC INDEPENDANT SUPPORTED",
    description: "AC independent feature supported by this machine.",
};

/**
 * Dynasty/Maxstar 210/280 Models Only. Volt Sensing (MIG) Model Detect: 1 True / 0 False.
 */
pub const IS_MIG_VOLT_SENSE_MODEL: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2016 },
    name: "IS MIG VOLT SENSE MODEL",
    description: "MIG volt-sensing model detected (210/280 only).",
};

/**
 * Syncrowave Model Detect: 1 True / 0 False.
 */
pub const IS_SYNCROWAVE_MODEL: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2017 },
    name: "IS SYNCROWAVE MODEL",
    description: "Syncrowave model detected.",
};

/**
 * Syncrowave 300/400 Models Only. Non Cooler Supply Detect: 1 True / 0 False.
 */
pub const NON_COOLER_SUPPLY_DETECT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2018 },
    name: "NON COOLER SUPPLY DETECT",
    description: "Non-cooler supply detected (Syncrowave 300/400 only).",
};

/**
 * 32 bit.
 * Dynasty/Maxstar 800 Models Only.
 * Application Software Number and Revision, 4 bytes bit mapped:
 * NNNN,NNNN NNNN,NNNN NNNN,NNRR RRRE,EEEE
 * NNNN,NNNN NNNN,NNNN NNNN,NN == Miller Part Number, 22 bits 31-10, bit range 0-4,194,303,
 * actual 0-999999.
 * RR RRR == Revision Level, 5 bits 9-5, bit range 0-31, actual 0-26 where: 0 == "@" preproduction
 * or field test software; 1,2,3... == Revision A,B,C...
 * EEEE == Evaluation / Test, 5 bits 4-0, bit range 0-31, actual 0-26 where: 0 == "@" released
 * software; 1,2,3... == Evaluation / Test Revision A,B,C...
 *
 * Context: Displayed as "Part Number: <n>, Release Revision: <letter>, Evaluation Revision: <letter>".
 */
pub const APP_SOFTWARE_VERSION_PCB_7: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4016 },
    name: "APP SOFTWARE VERSION PCB 7",
    description: "Application software version for PCB 7 (output control), shown as part number and revision letters.",
};

/**
 * 32 bit.
 * Application Software Number and Revision, PCB 6 Gateway Interface.
 * See APP_SOFTWARE_VERSION_PCB_7.
 *
 * Context: Displayed as "Part Number: <n>, Release Revision: <letter>, Evaluation Revision: <letter>".
 */
pub const APP_SOFTWARE_VERSION_PCB_6: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4018 },
    name: "APP SOFTWARE VERSION PCB 6",
    description: "Application software version for PCB 6 (gateway interface), shown as part number and revision letters.",
};

/**
 * 32 bit.
 * Dynasty/Maxstar 210/280 Models Only. Application Software Number and Revision, PCB 5 Cooler
 * Power Supply (CPS).
 * See APP_SOFTWARE_VERSION_PCB_7.
 *
 * Context: Displayed as "Part Number: <n>, Release Revision: <letter>, Evaluation Revision: <letter>".
 */
pub const APP_SOFTWARE_VERSION_PCB_5: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4020 },
    name: "APP SOFTWARE VERSION PCB 5",
    description: "Application software version for PCB 5 (cooler power supply), shown as part number and revision letters.",
};

/**
 * 32 bit.
 * Application Software Number and Revision, PCB 4 Primary.
 * See APP_SOFTWARE_VERSION_PCB_7.
 *
 * Context: Displayed as "Part Number: <n>, Release Revision: <letter>, Evaluation Revision: <letter>".
 */
pub const APP_SOFTWARE_VERSION_PCB_4: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4022 },
    name: "APP SOFTWARE VERSION PCB 4",
    description: "Application software version for PCB 4 (primary), shown as part number and revision letters.",
};

/**
 * 32 bit.
 * Application Software Number and Revision, PCB 3 Process.
 * See APP_SOFTWARE_VERSION_PCB_7.
 *
 * Context: Displayed as "Part Number: <n>, Release Revision: <letter>, Evaluation Revision: <letter>".
 */
pub const APP_SOFTWARE_VERSION_PCB_3: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4024 },
    name: "APP SOFTWARE VERSION PCB 3",
    description: "Application software version for PCB 3 (process), shown as part number and revision letters.",
};

/**
 * 32 bit.
 * Application Software Number and Revision, PCB 2 User Interface.
 * See APP_SOFTWARE_VERSION_PCB_7.
 *
 * Context: Displayed as "Part Number: <n>, Release Revision: <letter>, Evaluation Revision: <letter>".
 */
pub const APP_SOFTWARE_VERSION_PCB_2: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4026 },
    name: "APP SOFTWARE VERSION PCB 2",
    description: "Application software version for PCB 2 (user interface), shown as part number and revision letters.",
};

/**
 * 32 bit.
 * Application Software Number and Revision, PCB 1 SD Card.
 * See APP_SOFTWARE_VERSION_PCB_7.
 *
 * Context: Displayed as "Part Number: <n>, Release Revision: <letter>, Evaluation Revision: <letter>".
 */
pub const APP_SOFTWARE_VERSION_PCB_1: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4028 },
    name: "APP SOFTWARE VERSION PCB 1",
    description: "Application software version for PCB 1 (SD card), shown as part number and revision letters.",
};

/**
 * 32 bit.
 * Serial Number: 4 bytes bit mapped:
 * DDDY,YYYW WWWW,WSSS,SSSS,SSSS,SSSB,BBBB
 * DDD = Decade Code, 3 bits 31-29, bit range 0-7, actual "M" - "U" (for decades 201*-208*),
 * skip "O".
 * Y,YYY = Year Code, 4 bits 28-25, bit range 0-15, actual 0-9 "A" - "K", skip "I".
 * W WWWW,W = Week Number, 6 bits 24-19, bit range 0-63, actual 01-52.
 * SSS,SSSS,SSSS SSS = Serialized Number, 14 bits 18-5, bit range 0-16383, actual 0001-9999.
 * B,BBBB = Business Unit Code, 5 bits 4-0, bit range 0-31, actual 0-25 "A"-"Z".
 * Note: Letters "I" and "O" are skipped in decade and year. Not used in business unit code.
 *
 * Context: Displayed as a decoded serial like "MY051234B" (decade, year, week, serial, business unit).
 */
pub const SERIAL_NUMBER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4030 },
    name: "SERIAL NUMBER",
    description: "Machine serial number as decoded text (decade, year, week, serial, business unit).",
};

/**
 * Power Source Configuration, Amperage Maximum: 0-1023, Res: 1A.
 */
pub const MAX_AMPS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4032 },
    name: "MAX AMPS",
    description: "Maximum output amperage configured for this power source.",
};

/**
 * Power Source Configuration, Amperage DC Minimum: 0-31, Res: 1A, 0 = DC Not Available.
 */
pub const MIN_DC_AMPS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4033 },
    name: "MIN DC AMPS",
    description: "Minimum DC amperage configured for this power source.",
};

/**
 * Power Source Configuration, Amperage AC Minimum: 0-31, Res: 1A, 0 = AC Not Available.
 */
pub const MIN_AC_AMPS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4034 },
    name: "MIN AC AMPS",
    description: "Minimum AC amperage configured for this power source.",
};

/**
 * 32 bit.
 * Machine's Software Update Number, Revision. 4 bytes bit mapped:
 * NNNN,NNNN,NNNN,NNNN,NNNN,NNMM,MMML,LLLL
 * NNNN,NNNN,NNNN,NNNN,NNNN,NN == Miller Part Number, 22 bits 31-10, bit range 0-4,194,303,
 * actual 0-999999.
 * MM MMM = Revision Level's Most Significant Designator, 5 bits 9-5, bit range 0-31,
 * actual 0,1-26 (ASCII "@,A-Z"), 9 "I" and 15 "O" similar to "1" and "0" not used.
 * Typically starts at 0 ("@", omitted when displayed), increases by one with each wrap "Z" to "A"
 * of the least significant designator.
 * L, LLLL = Revision Level's Least Significant Designator, 5 bits 4-0, bit range 0-31,
 * actual 0,1-26 (ASCII "@,A-Z"), 9 "I" and 15 "O" similar to "1" and "0" not used.
 * 0 "@" used for preproduction only.
 *
 * Context: Displayed as "Part Number: <n>, Release Revision: <letters>".
 */
pub const SOFTWARE_VERSION: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4036 },
    name: "SOFTWARE VERSION",
    description: "Machine software update version, shown as part number and revision letters.",
};

/**
 * Sequence Timer: Remaining / Elapsed Time of States:
 * Initial Amperage, Initial Slope Time, Main Amperage, Final Slope Time, Final Amperage, Preflow,
 * Postflow (typically timed while in Standby State).
 * Resolution: 0.1 second.
 */
pub const SEQUENCE_TIMER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4099 },
    name: "SEQUENCE TIMER",
    description: "Remaining or elapsed time for the current weld sequence phase.",
};

/**
 * State:
 * 0 Initial Amperage
 * 1 Initial Slope Time
 * 2 Main Amperage
 * 3 Final Slope Time
 * 4 Final Amperage
 * 5 Preflow
 * 6 Standby
 * 7 Output Shorted
 * 8 Release Trigger
 * 9 Output Disabled
 * 13 Error
 * 14 Power Down
 * 15 Power Up
 */
pub const WELD_STATE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4100 },
    name: "WELD STATE",
    description: "Current weld sequence state (preflow, initial, main, final, standby, etc.).",
};

/**
 * Errors 1, 16 bits. Possible errors, 1 True / 0 False (power source dependent).
 * Dynasty/Maxstar 210/280, Syncrowave 300 Process and User Interface:
 * - Bit 0: 0.3.1 Secondary Over Temp
 * - Bit 1: 0.3.2 Ambient Over Temp
 * - Bit 2: 7.3.6 Process Serial Communication With Gateway
 * - Bit 3: 3.3.1 Secondary Thermistor Failure
 * - Bit 4: 3.3.2 Ambient Thermistor Failure
 * - Bit 5: 1.3.1 Fan Failure
 * - Bit 6: 1.3.2 Clamp/Output Over Voltage
 * - Bit 7: 1.3.3 AC Commutation Time Out
 * - Bit 8: 1.3.4 Output Over Voltage
 * - Bit 9: 1.3.5 Output Current Or Voltage Feedback With Output Off
 * - Bit 10: 1.3.6 No Cooler Detected With Output Current
 * - Bit 11: 7.3.4 Process Serial Communication With Primary
 * - Bit 12: 7.3.2 Process Serial Communication With User Interface
 * - Bit 13: 7.3.1 Process Serial Communication With Memory Card
 * - Bit 14: 7.3.5 Process Serial Communication With CPS
 * - Bit 15: 7.2.3 User Interface Serial Communication With Process
 * Dynasty/Maxstar 400/800:
 * - Bit 0: 0.3.2 Ambient Over Temp
 * - Bit 1: 0.3.1 Secondary Over Temp RC20
 * - Bit 2: 0.3.1 Secondary Over Temp RC30
 * - Bit 3: 0.4.1 Primary Power Over Temp 400/800 Top
 * - Bit 4: 0.4.2 or 0.7.1 Primary Power Over Temp 800 Bottom
 * - Bit 5: (unused)
 * - Bit 6: (unused)
 * - Bit 7: (unused)
 * - Bit 8: (unused)
 * - Bit 9: (unused)
 * - Bit 10: (unused)
 * - Bit 11: 7.3.7 Process serial communication with Primary 800 Bottom
 * - Bit 12: 7.3.4 Process serial communication with Primary 400/800 Top
 * - Bit 13: 3.3.2 Ambient thermistor failure
 * - Bit 14: 3.3.1 Secondary thermistor failure RC20
 * - Bit 15: 3.3.1 Secondary thermistor failure RC30
 */
pub const ERROR_REG_1: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4101 },
    name: "ERROR REG 1",
    description: "Active fault flags for system errors (decoded in the error list).",
};

/**
 * Errors 2, 16 bits. Possible errors, 1 True / 0 False (power source dependent).
 * Dynasty/Maxstar 210/280, Syncrowave 300 Primary:
 * - Bit 0: 0.4.1 Primary Power 1 Over Temp
 * - Bit 1: 0.4.2 Primary Power 2 Over Temp
 * - Bit 2: 1.4.8 Ground Current
 * - Bit 3: 1.4.0 Primary Not Ready
 * - Bit 4: 1.4.1 Primary Capacitor Imbalance
 * - Bit 5: 1.4.2 Input Over Voltage
 * - Bit 6: 1.4.3 Input Over Current
 * - Bit 7: 1.4.4 Primary Bus Under Voltage
 * - Bit 8: 1.4.5 Input Under Voltage
 * - Bit 9: 3.4.1 Primary Power 1 Thermistor Failure
 * - Bit 10: 3.4.2 Primary Power 2 Thermistor Failure
 * - Bit 11: 7.4.3 Primary Serial Communication With Process
 * - Bit 12: 1.4.6 Primary Capacitor Failure
 * - Bit 13: 1.4.7 Primary Control Power
 * - Bit 14: 0.4.1L Primary Power 1 Latched Over Temp
 * - Bit 15: 0.4.2L Primary Power 2 Latched Over Temp
 * Dynasty/Maxstar 400/800, Syncrowave 400:
 * - Bit 0: 3.4.1 Primary Power Thermistor Failure 400/800 Top
 * - Bit 1: 3.4.2 or 3.7.1 Primary Power Thermistor Failure 800 Bottom
 * - Bit 2: 1.3.2 Clamp/Output over voltage
 * - Bit 3: 1.3.3 AC Communication time out
 * - Bit 4: 1.3.4 Output over voltage
 * - Bit 5: 1.3.5 Output current or voltage feedback with output off
 * - Bit 6: 1.4.8 Ground current
 * - Bit 7: 1.4.3 Input over current 400/800 Top
 * - Bit 8: 1.4.3 or 1.7.3 Input over current 800 Bottom
 * - Bit 9: 1.4.7 Primary control power
 * - Bit 10: 1.4.5 Input under voltage
 * - Bit 11: 1.4.4 Primary bus under voltage
 * - Bit 12: 7.3.6 Process serial communication with Gateway
 * - Bit 13: 7.3.2 Process serial communication with User Interface
 * - Bit 14: 7.3.1 Process serial communication with Memory Card
 * - Bit 15: 7.2.3 User interface serial communication with Process
 */
pub const ERROR_REG_2: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4102 },
    name: "ERROR REG 2",
    description: "Active fault flags for primary power and communications errors (decoded in the error list).",
};

/**
 * Errors 3, 16 bits. Possible errors, 1 True / 0 False (power source dependent).
 * Dynasty Maxstar 210 and 280 CPS:
 * - Bit 0: 0.5.1 CPS Power Module 1 Over Temp
 * - Bit 1: 0.5.2 CPS Power Module 2 Over Temp
 * - Bit 2: 0.5.3 CPS Power Module 3 Over Temp
 * - Bit 3: 1.5.9 CPS Primary Bus Under Voltage
 * - Bit 4: 7.5.3 CPS Serial Communication With Process
 * - Bit 5: 3.5.1 CPS Power Module 1 Thermistor Failure
 * - Bit 6: 3.5.2 CPS Power Module 2 Thermistor Failure
 * - Bit 7: 3.5.3 CPS Power Module 3 Thermistor Failure
 * - Bit 8: 1.5.1 CPS Secondary Bus Under Voltage
 * - Bit 9: 1.5.2 CPS Output Over Current
 * - Bit 10: 1.5.3 CPS Secondary Bus Over Voltage
 * - Bit 11: 1.5.4 CPS Current Or Voltage feedback With CPS off
 * - Bit 12: 1.5.5 CPS Secondary Control Power
 * - Bit 13: 1.5.6 CPS Capacitor Imbalance
 * - Bit 14: 1.5.7 CPS Primary Control Power
 * - Bit 15: 1.5.8 CPS Secondary Communication With CPS Primary
 * Syncrowave 300:
 * - Bit 3: 1.5.9 CPS Primary Bus Under Voltage
 * Dynasty/Maxstar 400/800, Syncrowave 400:
 * - Bit 0: 1.5.9 CPS Primary Bus Under Voltage
 * - Bit 1: 1.4.4 Primary Bus Under Voltage 400/800 Top
 * - Bit 2: 1.4.5 Input Under Voltage 400/800 Top
 * - Bit 3: 1.4.2 Input Over Voltage 400/800 Top
 * - Bit 4: 1.4.7 Primary Control Power 400/800 Top
 * - Bit 5: 7.4.3 Primary Serial Communication With Process 400/800 Top
 * - Bit 6: 1.4.0 Primary Not Ready 400/800 Top
 * - Bit 7: (unused)
 * - Bit 8: (unused)
 * - Bit 9: 1.7.4 Primary Bus Under Voltage 800 Bottom
 * - Bit 10: 1.7.5 Input Under Voltage 800 Bottom
 * - Bit 11: 1.7.2 Input Over Voltage 800 Bottom
 * - Bit 12: 1.7.7 Primary Control Power 800 Bottom
 * - Bit 13: 7.7.3 Primary Serial Communication With Process 800 Bottom
 * - Bit 14: 1.7.0 Primary Not Ready 800 Bottom
 * - Bit 15: (unused)
 */
pub const ERROR_REG_3: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4103 },
    name: "ERROR REG 3",
    description: "Active fault flags for cooler power supply and primary errors (decoded in the error list).",
};

/**
 * Power Source Command Out Amperage, Res: 1A.
 */
pub const COMMANDED_OUTPUT_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4200 },
    name: "COMMANDED OUTPUT AMPERAGE",
    description: "Commanded output current setpoint.",
};

/**
 * Power Source Output Current, Res: 1A.
 */
pub const OUTPUT_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4201 },
    name: "OUTPUT AMPERAGE",
    description: "Measured output current at the weld terminals.",
};

/**
 * Power Source Output Voltage, Res: 0.1V.
 */
pub const OUTPUT_VOLTAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4202 },
    name: "OUTPUT VOLTAGE",
    description: "Measured output voltage at the weld terminals.",
};

/**
 * Power Source Output Current DC Pulse Peak, Res: 1A.
 */
pub const OUTPUT_CURRENT_DC_PULSE_PEAK: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4203 },
    name: "OUTPUT CURRENT DC PULSE PEAK",
    description: "Measured output current during the DC pulse peak portion.",
};

/**
 * Power Source Output Voltage DC Pulse Peak, Res: 0.1V.
 */
pub const OUTPUT_VOLTAGE_DC_PULSE_PEAK: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4204 },
    name: "OUTPUT VOLTAGE DC PULSE PEAK",
    description: "Measured output voltage during the DC pulse peak portion.",
};

/**
 * Power Source Output Current DC Pulse Back, Res: 1A.
 */
pub const OUTPUT_CURRENT_DC_PULSE_BACK: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4205 },
    name: "OUTPUT CURRENT DC PULSE BACK",
    description: "Measured output current during the DC pulse background portion.",
};

/**
 * Power Source Output Voltage DC Pulse Back, Res: 0.1V.
 */
pub const OUTPUT_VOLTAGE_DC_PULSE_BACK: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4206 },
    name: "OUTPUT VOLTAGE DC PULSE BACK",
    description: "Measured output voltage during the DC pulse background portion.",
};

/**
 * Fan Out, 0 (Off) - 100%.
 */
pub const FAN_OUTPUT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4300 },
    name: "FAN OUTPUT",
    description: "Current fan output level.",
};


/**
 * Temperature registers (power source dependent):
 * Range: 0 - 254
 * Resolution: 1 C
 * Offset: -50 (i.e. 50 == 0 C)
 * Possible range: -50 - +204 C
 * Actual range: limited by thermistor's hardware and software
 *
 * Temperature 1:
 * - Dynasty/Maxstar 210/280, Syncrowave 300: Primary Power 1
 * - Dynasty/Maxstar 400/800, Syncrowave 400: Ambient
 */
pub const TEMPERATURE_1: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4301 },
    name: "TEMPERATURE 1",
    description: "Temperature reading in C. On 210/280 and Syncrowave 300 this is Primary Power 1; on 400/800 and Syncrowave 400 it is Ambient.",
};

/**
 * Temperature registers (power source dependent):
 * Range: 0 - 254
 * Resolution: 1 C
 * Offset: -50 (i.e. 50 == 0 C)
 * Possible range: -50 - +204 C
 * Actual range: limited by thermistor's hardware and software
 *
 * Temperature 2:
 * - Dynasty/Maxstar 210/280, Syncrowave 300: Primary Power 2
 * - Dynasty/Maxstar 400/800 Top, Syncrowave 400: Primary Power
 */
pub const TEMPERATURE_2: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4302 },
    name: "TEMPERATURE 2",
    description: "Temperature reading in C. On 210/280 and Syncrowave 300 this is Primary Power 2; on 400/800 top and Syncrowave 400 it is Primary Power.",
};

/**
 * Temperature registers (power source dependent):
 * Range: 0 - 254
 * Resolution: 1 C
 * Offset: -50 (i.e. 50 == 0 C)
 * Possible range: -50 - +204 C
 * Actual range: limited by thermistor's hardware and software
 *
 * Temperature 3:
 * - Dynasty/Maxstar 210/280, Syncrowave 300: Secondary
 * - Dynasty/Maxstar 800 Bottom: Primary Power
 */
pub const TEMPERATURE_3: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4303 },
    name: "TEMPERATURE 3",
    description: "Temperature reading in C. On 210/280 and Syncrowave 300 this is Secondary; on Dynasty 800 bottom it is Primary Power.",
};

/**
 * Temperature registers (power source dependent):
 * Range: 0 - 254
 * Resolution: 1 C
 * Offset: -50 (i.e. 50 == 0 C)
 * Possible range: -50 - +204 C
 * Actual range: limited by thermistor's hardware and software
 *
 * Temperature 4:
 * - Dynasty/Maxstar 210/280, Syncrowave 300: Ambient
 * - Dynasty/Maxstar 400/800, Syncrowave 400: Secondary RC20
 */
pub const TEMPERATURE_4: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4304 },
    name: "TEMPERATURE 4",
    description: "Temperature reading in C. On 210/280 and Syncrowave 300 this is Ambient; on 400/800 and Syncrowave 400 it is Secondary RC20.",
};

/**
 * Temperature registers (power source dependent):
 * Range: 0 - 254
 * Resolution: 1 C
 * Offset: -50 (i.e. 50 == 0 C)
 * Possible range: -50 - +204 C
 * Actual range: limited by thermistor's hardware and software
 *
 * Temperature 5:
 * - Dynasty/Maxstar 210/280: CPS Module 1
 * - Dynasty/Maxstar 400/800, Syncrowave 400: Secondary RC30
 */
pub const TEMPERATURE_5: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4305 },
    name: "TEMPERATURE 5",
    description: "Temperature reading in C. On 210/280 this is CPS Module 1; on 400/800 and Syncrowave 400 it is Secondary RC30.",
};

/**
 * Temperature registers (power source dependent):
 * Range: 0 - 254
 * Resolution: 1 C
 * Offset: -50 (i.e. 50 == 0 C)
 * Possible range: -50 - +204 C
 * Actual range: limited by thermistor's hardware and software
 *
 * Temperature 6:
 * - Dynasty/Maxstar 210/280: CPS Module 2
 */
pub const TEMPERATURE_6: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4306 },
    name: "TEMPERATURE 6",
    description: "Temperature reading in C. On 210/280 this is CPS Module 2.",
};

/**
 * Temperature registers (power source dependent):
 * Range: 0 - 254
 * Resolution: 1 C
 * Offset: -50 (i.e. 50 == 0 C)
 * Possible range: -50 - +204 C
 * Actual range: limited by thermistor's hardware and software
 *
 * Temperature 7:
 * - Dynasty/Maxstar 210/280: CPS Module 3
 */
pub const TEMPERATURE_7: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4307 },
    name: "TEMPERATURE 7",
    description: "Temperature reading in C. On 210/280 this is CPS Module 3.",
};

/**
 * Dynasty/Maxstar 210/280, Syncrowave 300 - Primary Line Current, Res: 1A.
 */
pub const PRIMARY_LINE_CURRENT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4400 },
    name: "PRIMARY LINE CURRENT",
    description: "Primary input line current (mains).",
};

/**
 * Dynasty/Maxstar 210/280, Syncrowave 300 - Primary Line Voltage, Res: 1V.
 */
pub const PRIMARY_LINE_VOLTAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4401 },
    name: "PRIMARY LINE VOLTAGE",
    description: "Primary input line voltage (mains).",
};

/**
 * Dynasty/Maxstar 210/280/400/800, Syncrowave 300/400 - Primary Line Voltage Peak, Res: 1V.
 */
pub const PRIMARY_LINE_VOLTAGE_PEAK: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4402 },
    name: "PRIMARY LINE VOLTAGE PEAK",
    description: "Primary line voltage peak (mains).",
};

/**
 * Dynasty/Maxstar 210/280/400/800, Syncrowave 300/400 - Primary Bus Voltage, Res: 1V.
 */
pub const PRIMARY_BUS_VOLTAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4403 },
    name: "PRIMARY BUS VOLTAGE",
    description: "Primary DC bus voltage.",
};

/**
 * Dynasty/Maxstar 210/280 - Cooler Power Output Voltage, Res: 1V.
 */
pub const COOLER_OUTPUT_VOLTAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4404 },
    name: "COOLER OUTPUT VOLTAGE",
    description: "Cooler power supply output voltage.",
};

/**
 * Dynasty/Maxstar 210/280 - Cooler Power Output Current, Res: 0.1A.
 */
pub const COOLER_OUTPUT_CURRENT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4405 },
    name: "COOLER OUTPUT CURRENT",
    description: "Cooler power supply output current.",
};

/**
 * Dynasty/Maxstar 210/280 - Cooler Power Bus Voltage, Res: 1V.
 */
pub const COOLER_BUS_VOLTAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4406 },
    name: "COOLER BUS VOLTAGE",
    description: "Cooler power supply bus voltage.",
};

/**
 * Dynasty/Maxstar 800 - Primary 2 (bottom) Line Voltage Peak, Res: 1V.
 */
pub const PRIMARY_2_LINE_VOLTAGE_PEAK: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4407 },
    name: "PRIMARY 2 LINE VOLTAGE PEAK",
    description: "Second primary line voltage peak (800 bottom section).",
};

/**
 * Dynasty/Maxstar 800 - Primary 2 (bottom) Bus Voltage Peak, Res: 1V.
 */
pub const PRIMARY_2_BUS_VOLTAGE_PEAK: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4408 },
    name: "PRIMARY 2 BUS VOLTAGE PEAK",
    description: "Second primary bus voltage peak (800 bottom section).",
};

/**
 * Power Source's Modbus Slave Address: 1 - 247.
 */
pub const MB_SLAVE_ADDR: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6000 },
    name: "MB SLAVE ADDR",
    description: "Modbus slave address for the power source on the RS-485 link.",
};

/**
 * Fan Request:
 * Dynasty/Maxstar 210/280: 0 (Off), 1 (Min 27%) - 30 (Max 100%).
 * Requires request of 3 minimum to start fan.
 * Dynasty/Maxstar 400/800: 0 (Off), 1 - 30 (Max 100%).
 * Notes: 1 second time out return to 0 (Off).
 * Parallel request with all machine thermistors; highest fan request is used.
 * 0 (Off) in this register will not turn fan off with a fan request other than Off from any
 * machine thermistors.
 */
pub const FAN_REQUEST: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6001 },
    name: "FAN REQUEST",
    description: "Requested fan speed or cooling demand; highest active request wins.",
};

/**
 * Meter Calibration, Amperage: +-50, Res: 0.1% (+-50 == +-5.0%).
 * Note: With Discrete Input 2012 RMS Hardware Detect = True, Coil 16 RMS Enable selects RMS (True)
 * or Average (False) amperage calibration.
 *
 * Context: RMS hardware detect is exposed here as discrete input 2012.
 */
pub const CURRENT_METER_CALIBRATION: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6002 },
    name: "CURRENT METER CALIBRATION",
    description: "Calibration trim for the current meter (service setting).",
};

/**
 * Meter Calibration, Voltage Average: +-50, Res: 0.1% (+-50 == +-5.0%).
 */
pub const VOLTAGE_METER_CALIBRATION: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6003 },
    name: "VOLTAGE METER CALIBRATION",
    description: "Calibration trim for the voltage meter (service setting).",
};

/**
 * 32 bit.
 * Arc Time, Res: 0.01 minute, Maximum: 59999999 == 9999 hours and 59.99 minutes.
 *
 * Context: Displayed as hours, minutes, seconds.
 */
pub const ARC_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6100 },
    name: "ARC TIME",
    description: "Total arc-on time (lifetime), displayed as hours, minutes, and seconds.",
};

/**
 * 32 bit.
 * Arc Cycles, Res: 1 cycle, Maximum: 999999 cycles.
 */
pub const ARC_CYCLES: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6102 },
    name: "ARC CYCLES",
    description: "Total arc starts/cycles (lifetime).",
};

/**
 * Dynasty/Maxstar 400/800 Models Only.
 * Memory: 0 Memory control off typically defaults to memory 1 with no memory number displayed.
 * 1 - Power Source's memory maximum.
 */
pub const MEMORY_CONTROL: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6200 },
    name: "MEMORY CONTROL",
    description: "Selects the active memory program (400/800 models only).",
};

/**
 * * Process:
 * 0 Stick
 * 1 TIG
 * 2 MIG (selectable only with Dynasty/Maxstar 210/280 models and Dynasty's polarity DC)
 * 3 Test
 * 4 Hot Wire
 */
pub const WELD_PROCESS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6201 },
    name: "WELD PROCESS",
    description: "Selects the weld process (Stick, TIG, MIG, Test, Hot Wire).",
};

/**
 * * Process Start: 0 Scratch, 1 Lift, 2 HF.
 */
pub const WELD_START_PROCESS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6202 },
    name: "WELD START PROCESS",
    description: "Selects the TIG start method (scratch, lift, or HF).",
};

/**
 * * Trigger:
 * 0 None - Output Off
 * 1 Panel - Output On
 * 2 Standard
 * 3 2T Hold
 * 4 3T Hold
 * 5 4T Hold
 * 6 4TL Mini Logic Hold
 * 7 4TE Momentary Hold
 * 8 4Tm Modified Hold
 */
pub const TRIGGER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6203 },
    name: "TRIGGER",
    description: "Selects the trigger mode for torch switch behavior (2T/3T/4T variants).",
};

/**
 * * Remote 14-skt E Configuration:
 * 0 Amperage Control (slow response, finger tip/foot controls)
 * 1 External Pulse Control (amperage, fast response)
 * 2 Output Enable (14-Skt E-D shorted enables power source output)
 * 3 Disable (14-Skt E has no function)
 */
pub const RMT_PIN_E_CONFIG: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6204 },
    name: "RMT PIN E CONFIG",
    description: "Configures Remote 14 pin E (amperage control, external pulse, output enable, or disabled).",
};

/**
 * * Tungsten (Canned Arc Start Parameters):
 * 0 0.020 in. (0.5 mm)
 * 1 0.040 in. (1.0 mm)
 * 2 1/16 in. (1.6 mm)
 * 3 3/32 in. (2.4 mm)
 * 4 1/8 in. (3.2 mm)
 * 5 5/32 in. (4.0 mm)
 * 6 3/16 in. (4.8 mm)
 * 7 1/4 in. (6.4 mm)
 * 8 General (user defined with holding registers 6207 through 6212)
 * <9 Power source dependent, typically used with process TIG
 * 9 Disabled (typically used with non-TIG processes)
 */
pub const TUNGSTEN_PRESET: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6205 },
    name: "TUNGSTEN PRESET",
    description: "Selects the tungsten preset (diameter-based or General/Disabled).",
};

/**
 * Preset Amperage Minimum: Power Source AC / DC Amperage Minimum - 25 A (Tungsten General) or
 * 63 A (Tungsten Disabled), Res: 1A.
 * Write only with Tungsten General or Disabled.
 */
pub const PRESET_MIN_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6206 },
    name: "PRESET MIN AMPERAGE",
    description: "Minimum preset amperage used with the selected tungsten preset mode.",
};

/**
 * Arc Start Amperage: 5 A - 200 A, Res: 1A.
 * Write only with Tungsten General or Disabled.
 */
pub const ARC_START_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6207 },
    name: "ARC START AMPERAGE",
    description: "Arc start amperage for TIG.",
};

/**
 * Arc Start Time: 0 (Off) - 25 (x10 ms), Res: 1 (x10 ms).
 * Write only with Tungsten General.
 */
pub const ARC_START_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6208 },
    name: "ARC START TIME",
    description: "Time at arc start amperage before ramping.",
};

/**
 * Arc Start Slope Time: 0 (Off) - 25 (x10 ms), Res: 1 (x10 ms).
 * Write only with Tungsten General.
 */
pub const ARC_START_SLOPE_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6209 },
    name: "ARC START SLOPE TIME",
    description: "Ramp time from arc start amperage to the next phase.",
};

/**
 * ** Arc Start AC Time: 0 (Off) - 25 (x10 ms), Res: 1 (x10 ms).
 * Write only with AC power source's AC output and Tungsten General.
 */
pub const ARC_START_AC_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6210 },
    name: "ARC START AC TIME",
    description: "AC-specific arc start time for TIG.",
};

/**
 * ** Arc Start Polarity Phase: 1 EP, 0 EN.
 * Write only with AC power source and Tungsten General or Disabled.
 */
pub const ARC_START_POLARITY_PHASE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6211 },
    name: "ARC START POLARITY PHASE",
    description: "Polarity used at arc start for AC TIG (EP or EN).",
};

/**
 * * AC EN Wave Shape: 0 Advance Square, 1 Soft Square, 2 Sine, 3 Triangle.
 */
pub const AC_EN_WAVE_SHAPE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6212 },
    name: "AC EN WAVE SHAPE",
    description: "Wave shape for the EN (electrode negative) portion of AC.",
};

/**
 * * AC EP Wave Shape: 0 Advance Square, 1 Soft Square, 2 Sine, 3 Triangle.
 */
pub const AC_EP_WAVE_SHAPE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6213 },
    name: "AC EP WAVE SHAPE",
    description: "Wave shape for the EP (electrode positive) portion of AC.",
};

/**
 * Hot Start Time:
 * Range: 0 (Off) - 20
 * Resolution: 0.1 second
 * Hot Start Enable / Disabled with Coil 8 Hot Start Enable.
 * Stick only.
 */
pub const HOT_START_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6214 },
    name: "HOT START TIME",
    description: "Duration of hot start boost for stick welding.",
};

/**
 * Remote Hold:
 * 0 / 2T
 * 1 / 3T
 * 2 / 4T
 * 3 / 4TL Mini Logic
 * 4 / 4TE Momentary
 * 5 / 4Tm Modified
 * Resolution: 0.1 second.
 * Remote Hold can also be changed with Holding Register 6204 Trigger.
 *
 * Context: Trigger mode is holding register 6203 in this project.
 */
pub const REMOTE_HOLD: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6215 },
    name: "REMOTE HOLD",
    description: "Remote hold mode for trigger behavior (2T/3T/4T variants).",
};

/**
 * * Dig, 0 (Off) - 100%, Res: 1%.
 * 101% will set Process Stick for Carbon Arc Gouging, turning Dig off and disabling Boost
 * (Coil 0009).
 * With Process (Holding Register 6201) MIG selection:
 * * Inductance 0 - 99% Res: 1%
 * 100% will set Inductance and optimize Digital Voltage Control for Flux Core Wire.
 */
pub const DIG_PERCENT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6217 },
    name: "DIG PERCENT",
    description: "Stick arc force (dig) or MIG inductance, depending on process.",
};

/**
 * * AC EN Amperage, Preset Amps Min - PS Amps Max, Res: 1A.
 */
pub const AC_EN_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6300 },
    name: "AC EN AMPERAGE",
    description: "Amperage for the EN portion of AC (independent AC settings).",
};

/**
 * * AC EP Amperage, Preset Amps Min - PS Amps Max, Res: 1A.
 */
pub const AC_EP_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6301 },
    name: "AC EP AMPERAGE",
    description: "Amperage for the EP portion of AC (independent AC settings).",
};

/**
 * * AC Balance, 30-99%, Res: 1%.
 */
pub const AC_BALANCE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6302 },
    name: "AC BALANCE",
    description: "AC balance (EN/EP ratio).",
};

/**
 * * AC Frequency, 20-400 Hz, Res: 1 Hz.
 */
pub const AC_FREQUENCY: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6303 },
    name: "AC FREQUENCY",
    description: "AC frequency setting.",
};

/**
 * * Weld Amperage (DC or AC), Preset Amps Min - PS Amps Max, Res: 1A.
 */
pub const WELD_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6304 },
    name: "WELD AMPERAGE",
    description: "Main weld amperage setpoint.",
};

/**
 * * Pulser - Pulses Per Second (PPS):
 * Range: 0 (Off) - 50000 / 5000 (power source dependent)
 * Resolution: 0.1 Hz
 * Can be set to a default value when writing a True to coil 18 Pulser Enable and PPS is found at
 * 0 (Off).
 * Writing a non "0" value will set coil 18 Pulser Enable to True.
 * Writing a "0" value will set coil 18 Pulser Enable to False.
 * Dependent on configuration of the slave, the slave may or may not retain the PPS non "0" value.
 */
pub const PULSER_PPS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6305 },
    name: "PULSER PPS",
    description: "Pulse frequency in pulses per second.",
};

/**
 * * Pulser - Peak Time, 5-95%, Res: 1%.
 */
pub const PULSER_PEAK_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6306 },
    name: "PULSER PEAK TIME",
    description: "Percent of the pulse spent at peak current.",
};

/**
 * * Pulser - Background Amperage, 5-95%, Res: 1%.
 */
pub const PULSER_BACKGROUND_AMPS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6307 },
    name: "PULSER BACKGROUND AMPS",
    description: "Background current level during pulsed welding.",
};

/**
 * * Prelow Time, 0 (Off) - 250, Res: 1 (x0.1 sec).
 * (Miller says prelow, assuming preflow.)
 */
pub const PREFLOW_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6308 },
    name: "PRE-FLOW TIME",
    description: "Gas preflow time before arc start.",
};

/**
 * * Initial Amperage, Preset Amps Min - PS Amps Max, Res: 1A.
 */
pub const INITIAL_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6309 },
    name: "INITIAL AMPERAGE",
    description: "Initial amperage for the weld sequence.",
};

/**
 * * Initial Time, 0 (Off) - 250, Res: 1 (x0.1 sec).
 */
pub const INITIAL_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6310 },
    name: "INITIAL TIME",
    description: "Time at initial amperage before ramping.",
};

/**
 * * Initial Slope Time, 0 (Off) - 500, Res: 1 (x0.1 sec).
 */
pub const INITIAL_SLOPE_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6311 },
    name: "INITIAL SLOPE TIME",
    description: "Ramp time from initial to main amperage.",
};

/**
 * * Main Time, 0 (Off) - 9990, Res: 1 (x0.1 sec).
 */
pub const MAIN_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6312 },
    name: "MAIN TIME",
    description: "Time at main amperage (spot/weld timer).",
};

/**
 * * Final Slope Time, 0 (Off) - 500, Res: 1 (x0.1 sec).
 */
pub const FINAL_SLOPE_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6313 },
    name: "FINAL SLOPE TIME",
    description: "Ramp time from main to final amperage.",
};

/**
 * * Final Amperage, Preset Amps Min - PS Amps Max, Res: 1A.
 */
pub const FINAL_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6314 },
    name: "FINAL AMPERAGE",
    description: "Final amperage for crater fill or end of weld.",
};

/**
 * * Final Time, 0 (Off) - 250, Res: 1 (x0.1 sec).
 */
pub const FINAL_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6315 },
    name: "FINAL TIME",
    description: "Time at final amperage before stopping output.",
};

/**
 * * Postflow Time, 0 (Off) - 50 s & Auto (51), Res: 1 sec.
 */
pub const POSTFLOW_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6316 },
    name: "POSTFLOW TIME",
    description: "Gas postflow time after arc end (Auto lets the machine decide).",
};

/**
 * * Dig, 0 (Off) - 100%, Res: 1%.
 * 101% will set Process Stick for Carbon Arc Gouging, turning Dig off and disabling Boost
 * (Coil 0009).
 *
 * Same as 6217??
 */
pub const DIG_PERCENT_AGAIN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6317 },
    name: "DIG PERCENT AGAIN",
    description: "Duplicate of dig/inductance setting (same meaning as DIG PERCENT).",
};

/**
 * * Hot Wire Voltage, 5-20, Res: 1V.
 */
pub const HOT_WIRE_VOLTAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6318 },
    name: "HOT WIRE VOLTAGE",
    description: "Hot wire voltage setpoint (Hot Wire process).",
};

pub const MILLER_REGISTERS: &'static[RegisterMetadata] = &[
    PS_UI_DISABLE,
    RMT_TRIGGER_DISABLE,
    CONTACTOR_TRIGGER,
    GAS_REQUEST,
    USE_DC_OUTPUT,
    USE_EP_POLARITY,
    STUCK_CHECK_EN,
    HOT_START_EN,
    BOOST_EN,
    DROOP_EN,
    USE_LOW_OCV,
    GAS_EN,
    COOLER_EN,
    COOLER_TIG_EN,
    COOLER_ERROR_EN,
    TOUCH_SENSE_EN,
    RMS_EN,
    PULSER_EN,
    USE_LOW_AC_COMMUTATION_AMP,
    AC_INDEPENDANT_EN,
    WLED_TIMERS_EN,
    COOLER_DETECTED,
    COOLER_LOAD_DETECTED,
    FOOT_CONTROL_DETECTED,
    RMT_TRIGGER_ENABLED,
    CONTACTOR_OUTPUT_ENABLED,
    GAS_OUTPUT_ENABLED,
    IS_VALID_ARC,
    ARC_LENGTH_CTL_LOCKOUT,
    TOUCH_SENSE_DETECT,
    IS_CE_MODEL,
    IS_STR_MODEL,
    IS_DX_MODEL,
    RMS_HW_PRESENT,
    LOW_LIVE_INPUT,
    HOT_START_SUPPORTED,
    AC_INDEPENDANT_SUPPORTED,
    IS_MIG_VOLT_SENSE_MODEL,
    IS_SYNCROWAVE_MODEL,
    NON_COOLER_SUPPLY_DETECT,
    APP_SOFTWARE_VERSION_PCB_7,
    APP_SOFTWARE_VERSION_PCB_6,
    APP_SOFTWARE_VERSION_PCB_5,
    APP_SOFTWARE_VERSION_PCB_4,
    APP_SOFTWARE_VERSION_PCB_3,
    APP_SOFTWARE_VERSION_PCB_2,
    APP_SOFTWARE_VERSION_PCB_1,
    SERIAL_NUMBER,
    MAX_AMPS,
    MIN_DC_AMPS,
    MIN_AC_AMPS,
    SOFTWARE_VERSION,
    SEQUENCE_TIMER,
    WELD_STATE,
    ERROR_REG_1,
    ERROR_REG_2,
    ERROR_REG_3,
    COMMANDED_OUTPUT_AMPERAGE,
    OUTPUT_AMPERAGE,
    OUTPUT_VOLTAGE,
    OUTPUT_CURRENT_DC_PULSE_PEAK,
    OUTPUT_VOLTAGE_DC_PULSE_PEAK,
    OUTPUT_CURRENT_DC_PULSE_BACK,
    OUTPUT_VOLTAGE_DC_PULSE_BACK,
    FAN_OUTPUT,
    TEMPERATURE_1,
    TEMPERATURE_2,
    TEMPERATURE_3,
    TEMPERATURE_4,
    TEMPERATURE_5,
    TEMPERATURE_6,
    TEMPERATURE_7,
    PRIMARY_LINE_CURRENT,
    PRIMARY_LINE_VOLTAGE,
    PRIMARY_LINE_VOLTAGE_PEAK,
    PRIMARY_BUS_VOLTAGE,
    COOLER_OUTPUT_VOLTAGE,
    COOLER_OUTPUT_CURRENT,
    COOLER_BUS_VOLTAGE,
    PRIMARY_2_LINE_VOLTAGE_PEAK,
    PRIMARY_2_BUS_VOLTAGE_PEAK,
    MB_SLAVE_ADDR,
    FAN_REQUEST,
    CURRENT_METER_CALIBRATION,
    VOLTAGE_METER_CALIBRATION,
    ARC_TIME,
    ARC_CYCLES,
    MEMORY_CONTROL,
    WELD_PROCESS,
    WELD_START_PROCESS,
    TRIGGER,
    RMT_PIN_E_CONFIG,
    TUNGSTEN_PRESET,
    PRESET_MIN_AMPERAGE,
    ARC_START_AMPERAGE,
    ARC_START_TIME,
    ARC_START_SLOPE_TIME,
    ARC_START_AC_TIME,
    ARC_START_POLARITY_PHASE,
    AC_EN_WAVE_SHAPE,
    AC_EP_WAVE_SHAPE,
    HOT_START_TIME,
    REMOTE_HOLD,
    DIG_PERCENT,
    AC_EN_AMPERAGE,
    AC_EP_AMPERAGE,
    AC_BALANCE,
    AC_FREQUENCY,
    WELD_AMPERAGE,
    PULSER_PPS,
    PULSER_PEAK_TIME,
    PULSER_BACKGROUND_AMPS,
    PREFLOW_TIME,
    INITIAL_AMPERAGE,
    INITIAL_TIME,
    INITIAL_SLOPE_TIME,
    MAIN_TIME,
    FINAL_SLOPE_TIME,
    FINAL_AMPERAGE,
    FINAL_TIME,
    POSTFLOW_TIME,
    DIG_PERCENT_AGAIN,
    HOT_WIRE_VOLTAGE,
];


pub fn get_miller_register_metadata(register_name: &str) -> Option<&'static RegisterMetadata> {
    static REGISTER_MAP: std::sync::OnceLock<std::collections::HashMap<&'static str, &'static RegisterMetadata>> = std::sync::OnceLock::new();

    let map = REGISTER_MAP.get_or_init(|| {
        MILLER_REGISTERS.iter().map(|r| (r.name, r)).collect()
    });
    
    map.get(register_name).copied()
}
