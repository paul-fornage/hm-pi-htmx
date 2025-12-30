use crate::modbus::{ModbusAddressType, RegisterAddress, RegisterMetadata};



pub const PS_UI_DISABLE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0000 },
    name: "PS_UI_DISABLE",
    description: "True disable the UI on the power supply. This is necessary to modify some regs with '*'",
};

pub const RMT_TRIGGER_DISABLE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0001 },
    name: "RMT_TRIGGER_DISABLE",
    description: "True disables the remote trigger on the 14 pin connector",
};

pub const CONTACTOR_TRIGGER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0002 },
    name: "CONTACTOR_TRIGGER",
    description: "Trigger (Contactor) Request: 1 True(1 Second Time Out Return To False) / 0 False. To continue a weld sequence through Final Slope and or Final Time, Coil must be refreshed with False throughout these sequences.",
};

pub const GAS_REQUEST: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0003 },
    name: "GAS_REQUEST",
    description: "Gas Request: 1 TRUE(1 Second Time Out Return To False) / 0 False.",
};

pub const USE_DC_OUTPUT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0004 },
    name: "USE_DC_OUTPUT",
    description: "*,**AC Power Source’s Output DC: 1 True (DC) / 0 False (AC).",
};

pub const USE_EP_POLARITY: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0005 },
    name: "USE_EP_POLARITY",
    description: "*,**AC Power Source’s DC Polarity EP: 1 True (EP) / 0 False (EN).",
};

pub const STUCK_CHECK_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0006 },
    name: "STUCK_CHECK_EN",
    description: "*Stuck Check Enable: 1 True / 0 False. When Stick Stuck Check is on and the welding electrode (rod) is stuck, output is turned off.",
};

pub const HOT_START_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0007 },
    name: "HOT_START_EN",
    description: "*Hot Start Enable: 1 True / 0 False.Note: Hot Start can also be Disabled with 0 time set in Holding Register 6214 Hot Start Time. Stick only",
};

pub const BOOST_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0008 },
    name: "BOOST_EN",
    description: "*Boost Enable: 1 True / 0 False.",
};

pub const DROOP_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0009 },
    name: "DROOP_EN",
    description: "*Droop Enable: 1 True / 0 False.",
};

pub const USE_LOW_OCV: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0010 },
    name: "USE_LOW_OCV",
    description: "*Open Circuit Voltage (OCV) Low Enable: 1 True (Low) / 0 False (Normal). OCV selection applies to both Stick and MIG processes.",
};

pub const GAS_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0011 },
    name: "GAS_EN",
    description: "*Weld Gas Enable: 1 True / 0 False Enables Gas With Contactor. Set true for automatic gas, false if using our own gas solenoid",
};

pub const COOLER_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0012 },
    name: "COOLER_EN",
    description: "Non CE Models Only Cooler Power Supply (CPS) Enable: 1 True (Parallel With Coil 0013) / 0 False. Note: Dynasty/Maxstar 210/280 CE Models Have No Control, Read Returns False. Cooler power supply is switched 120v for the cooler in the back.",
};

pub const COOLER_TIG_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0013 },
    name: "COOLER_TIG_EN",
    description: "*Cooler Power Supply (CPS) TIG Enable: 1 True (Parallel With Coil 0012) / 0 False TIG Process Control Of Cooler Power Supply.",
};

pub const COOLER_ERROR_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0014 },
    name: "COOLER_ERROR_EN",
    description: "Dynasty/Maxstar 210/280 Models Only *Cooler Error Enable: 1 True / 0 False Enables Error “1.3.6 No Cooler Detected With Output Current”. Error Is Generated When No Load Detected On Cooler Power Supply’s Output With Load Detected On The Power Source’s Output.",
};

pub const TOUCH_SENSE_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0015 },
    name: "TOUCH_SENSE_EN",
    description: "Touch Sense Enable: 1 True / 0 False. Touch Sense Detection found at Modbus Discrete Input 2008 Or Remote 14 Receptacle Socket J.",
};

pub const RMS_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0016 },
    name: "RMS_EN",
    description: "RMS Enable: AC Amperage Preset And Meter And/Or DC Pulse Amperage Meter:1 True (RMS) / 0 False (Average) Note: To Enable, Must Have Discrete Input 2012 RMS Hardware Detect = True.",
};

pub const PULSER_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0017 },
    name: "PULSER_EN",
    description: "*Pulser Enable: 1 True / 0 False. Note: Can also be set TRUE / FALSE when writing values to Holding Register 6305 Pulser Pulses Per Second (PPS). When enabled and Holding Register 6304 PPS is found at “0”, PPS will be set to a default value.",
};

pub const USE_LOW_AC_COMMUTATION_AMP: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0018 },
    name: "USE_LOW_AC_COMMUTATION_AMP",
    description: "Dynasty/Maxstar 400/800 Models Only *AC Commutation Amperage LOW ENABLE: 1 TRUE (LOW) / 0 FALSE (High) Application: Use High commutation amperage when a more aggressive arc is preferred. Use Low commutation amperage when a less aggressive and quieter arc is preferred.",
};

pub const AC_INDEPENDANT_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0019 },
    name: "AC_INDEPENDANT_EN",
    description: "*AC Independent Enable: 1 True / 0 False. Enables/Disables Both Independent Amperage and Independent AC Wave Shapes.",
};

pub const WLED_TIMERS_EN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::Coil, address: 0020 },
    name: "WLED_TIMERS_EN",
    description: "*Weld Timers Enable: 1 True / 0 False. Weld Timers Include Weld (Spot), Intial Amperage and Final Amperage Timers.",
};

pub const COOLER_DETECTED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2000 },
    name: "COOLER_DETECTED",
    description: "Dynasty/Maxstar 210/280 Models OnlyCooler Power Supply (CPS) Detect: 1 True / 0 False.",
};

pub const COOLER_LOAD_DETECTED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2001 },
    name: "COOLER_LOAD_DETECTED",
    description: "Dynasty/Maxstar 210/280 Models OnlyCooler Load Detect: 1 True / 0 False.",
};

pub const FOOT_CONTROL_DETECTED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2002 },
    name: "FOOT_CONTROL_DETECTED",
    description: "Foot/Finger Tip Control Detect: 1 True / 0 False Note: Holding Register 6204 (Remote 14-Skt E) Must Be Con-figured To 0 (Amperage Control) To Detect Foot/Finger Tip Control.",
};

pub const RMT_TRIGGER_ENABLED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2003 },
    name: "RMT_TRIGGER_ENABLED",
    description: "Remote Trigger (Contactor 14-Skt A-B) Enable: 1 True / 0 False. Not clear if this shows when it is enabled (like coil 1) Or if it shows when the trigger contactor is actually engaged.",
};

pub const CONTACTOR_OUTPUT_ENABLED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2004 },
    name: "CONTACTOR_OUTPUT_ENABLED",
    description: "Contactor Output Enabled: 1 True / 0 False (Contactor Output Or Sense Voltage Pre Contactor Output).",
};

pub const GAS_OUTPUT_ENABLED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2005 },
    name: "GAS_OUTPUT_ENABLED",
    description: "Gas Output Enabled: 1 True / 0 False.",
};

pub const IS_VALID_ARC: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2006 },
    name: "IS_VALID_ARC",
    description: "Valid Arc: 1 True / 0 False.",
};

pub const ARC_LENGTH_CTL_LOCKOUT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2007 },
    name: "ARC_LENGTH_CTL_LOCKOUT",
    description: "Arc Length Control Lock Out: 1 True / 0 False.",
};

pub const TOUCH_SENSE_DETECT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2008 },
    name: "TOUCH_SENSE_DETECT",
    description: "Touch Sense Detect: 1 True / 0 False. Touch Sense Enable (Coil 16) Must Be Set True With Machine’s State(Input Register 4101) In Standby, And Weld Output Shorted For Touch Sense Detect To Register As True.",
};

pub const IS_CE_MODEL: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2009 },
    name: "IS_CE_MODEL",
    description: "CE Model Detect: 1 True / 0 False",
};

pub const IS_STR_MODEL: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2010 },
    name: "IS_STR_MODEL",
    description: "STR Model Detect: 1 True / 0 False",
};

pub const IS_DX_MODEL: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2011 },
    name: "IS_DX_MODEL",
    description: "DX Model Detect: 1 True / 0 False",
};

pub const RMS_HW_PRESENT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2012 },
    name: "RMS_HW_PRESENT",
    description: "RMS Hardware Detect: 1 True / 0 False",
};

pub const LOW_LIVE_INPUT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2013 },
    name: "LOW_LIVE_INPUT",
    description: "Low Line Detect: 1 True / 0 False (Dynasty/Maxstar 210 Only)Note: Set True When Powered Up On 120 V Input.",
};

pub const HOT_START_SUPPORTED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2014 },
    name: "HOT_START_SUPPORTED",
    description: "Feature Enable for Hot Start Adjust: 1 True / 0 False.",
};

pub const AC_INDEPENDANT_SUPPORTED: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2015 },
    name: "AC_INDEPENDANT_SUPPORTED",
    description: "Feature Enable for AC Independent: 1 True / 0 False.",
};

pub const IS_MIG_VOLT_SENSE_MODEL: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2016 },
    name: "IS_MIG_VOLT_SENSE_MODEL",
    description: "Dynasty/Maxstar 210/280 Models OnlyVolt Sensing (MIG) Model Detect: 1 True / 0 False",
};

pub const IS_SYNCROWAVE_MODEL: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2017 },
    name: "IS_SYNCROWAVE_MODEL",
    description: "Syncrowave Model Detect: 1 True / 0 False",
};

pub const NON_COOLER_SUPPLY_DETECT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::DiscreteInput, address: 2018 },
    name: "NON_COOLER_SUPPLY_DETECT",
    description: "Syncrowave 300/400 Models Only. Non Cooler Supply Detect: 1 True / 0 False",
};

pub const APP_SOFTWARE_VERSION_PCB_7: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4016 },
    name: "APP_SOFTWARE_VERSION_PCB_7",
    description: "32 bit\n\nDynasty/Maxstar 800 Models Only\nApplication Software Number And Revision,4 Bytes Bit Mapped:\nNNNN,NNNN NNNN,NNNN NNNN,NNRR RRRE,EEEE\nNNNN,NNNN NNNN,NNNN NNNN,NN == Miller Part Number,22 Bits 31 - 10, Bit Range 0 - 4,194,303, Actual 0-999999\nRR RRR == Revision Level, 5 Bits 9 - 5, Bit Range 0 - 31,Actual 0 - 26where: 0 == “@” Preproduction Or Field Test Software1,2,3... == Revision A,B,C…E,\nEEEE == Evaluation / Test, 5 Bits 9 - 5, Bit Range 0 - 31,Actual 0 - 26Where: 0 == ”@” Released Software,1,2,3... == Evaluation / Test Revision A,B,C…PCB 7 Primary",
};

pub const APP_SOFTWARE_VERSION_PCB_6: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4018 },
    name: "APP_SOFTWARE_VERSION_PCB_6",
    description: "32 bit\n\nApplication Software Number And Revision,PCB 6 Gateway Interface\n\nSee APP_SOFTWARE_VERSION_PCB_7",
};

pub const APP_SOFTWARE_VERSION_PCB_5: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4020 },
    name: "APP_SOFTWARE_VERSION_PCB_5",
    description: "32 bit\n\nDynasty/Maxstar 210/280 Models OnlyApplication Software Number And Revision,PCB 5 Cooler Power Supply (CPS)\n\nSee APP_SOFTWARE_VERSION_PCB_7",
};

pub const APP_SOFTWARE_VERSION_PCB_4: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4022 },
    name: "APP_SOFTWARE_VERSION_PCB_4",
    description: "32 bit\n\nApplication Software Number And Revision,PCB 4 Primary\n\nSee APP_SOFTWARE_VERSION_PCB_7",
};

pub const APP_SOFTWARE_VERSION_PCB_3: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4024 },
    name: "APP_SOFTWARE_VERSION_PCB_3",
    description: "32 bit\n\nApplication Software Number And Revision,PCB 3 Process\n\nSee APP_SOFTWARE_VERSION_PCB_7",
};

pub const APP_SOFTWARE_VERSION_PCB_2: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4026 },
    name: "APP_SOFTWARE_VERSION_PCB_2",
    description: "32 bit\n\nApplication Software Number And Revision,PCB 2 User Interface\n\nSee APP_SOFTWARE_VERSION_PCB_7",
};

pub const APP_SOFTWARE_VERSION_PCB_1: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4028 },
    name: "APP_SOFTWARE_VERSION_PCB_1",
    description: "32 bit\n\nApplication Software Number And Revision,PCB 1 SD Card\n\nSee APP_SOFTWARE_VERSION_PCB_7",
};

pub const SERIAL_NUMBER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4030 },
    name: "SERIAL_NUMBER",
    description: "32 bit\n\nSerial Number: 4 Bytes Bit Mapped: \nDDDY,YYYW WWWW,WSSS,SSSS,SSSS,SSSB,BBBB\n\nDDD = Decade Code, 3 Bits 31 - 29, Bit Range 0 - 7, actual “M” - “U” (For Decades 201*-208*), Skip “O”, See Note \nY,YYY = Year Code, 4 Bits 28 - 25, Bit Range 0 - 15, Actual 0 - 9 “A” - “K”, Skip “I”, See Note\nW WWWW,W = Week Number, 6 Bits 24-19, Bit Range 0 - 63, Actual 01 - 52 \nSSS,SSSS,SSSS SSS = Serialized Number, 14 Bits 18 - 5, Bit Range 0 - 16383, Actual 0001-9999 \nB,BBBB = Business Unit Code, 5 Bits 4 - 0, Bit Range 0 - 31, Actual 0 - 25 “A”-”Z”, “I” And “O”, Not Used See Note Note: Letters “I” And “O”, Similar To Numbers “1” And “0” Skipped In Decade And Year. Not used In Business Unit Code.",
};

pub const MAX_AMPS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4032 },
    name: "MAX_AMPS",
    description: "Power Source Configuration, Amperage Maximum: 0-1023, Res: 1A. Unclear if this is for the machine or for the current configuration.",
};

pub const MIN_DC_AMPS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4033 },
    name: "MIN_DC_AMPS",
    description: "Power Source Configuration, Amperage DC Minimum: 0-31, Res: 1A, 0 = DC Not Available. Unclear if this is for the machine or for the current configuration.",
};

pub const MIN_AC_AMPS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4034 },
    name: "MIN_AC_AMPS",
    description: "Power Source Configuration, Amperage AC Minimum: 0-31, Res: 1A, 0 = AC Not Available. Unclear if this is for the machine or for the current configuration.",
};

pub const SOFTWARE_VERSION: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4036 },
    name: "SOFTWARE_VERSION",
    description: "32 bit\n\nMachine’s Software Update Number, Revision. 4 Bytes Bit Mapped: \nNNNN,NNNN,NNNN,NNNN,NNNN,NNMM,MMML,LLLL\nNNNN,NNNN,NNNN,NNNN,NNNN,NN = Miller Part Number, 22 Bits 31−10, Bit Range 0−4,194,303, Actual 0−999999 \nMM MMM = Revision Level’s Most Significant Designator, 5 Bits 9−5, Bit Range 0−31, Actual 0,1−26 (ASCII “@,A−Z”), 9 “I” & 15 “O” Similar To “1” & “0” Not Used. Typically Starts At 0 (“@”, Omitted When Displayed), Increases By One With Each Wrap “Z” To “A” Of The Least Significant Designator \nL, LLLL = Revision Level’s Least Significant Designator, 5 Bits 4−0, Bit Range 0−31, Actual 0,1−26 (ASCII “@,A−Z”), 9 “I” & 15 “O” Similar To “1” & “0” Not Used. 0 “@” \n\nUsed For Preproduction Only.",
};

pub const SEQUENCE_TIMER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4099 },
    name: "SEQUENCE_TIMER",
    description: "Sequence Timer: Remaining / Elapsed Time of States: Initial Amperage Initial Slope Time Main Amperage Final Slope Time Final Amperage Preflow Postflow (typically timed while in Standby State) Resolution: 0.1 Second",
};

pub const WELD_STATE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4100 },
    name: "WELD_STATE",
    description: "State: \n0 Initial Amperage\n1 Initial Slope Time\n2 Main Amperage\n3 Final Slope Time\n4 Final Amperage\n5 Preflow\n6 Standby\n7 Output Shorted\n8 Release Trigger\n9 Output Disabled\n13 Error\n14 Power Down\n15 Power Up",
};

pub const ERROR_REG_1: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4101 },
    name: "ERROR_REG_1",
    description: "Errors1, 16(Bits) Possible Errors, 1 True / 0 False (Power Source Dependent)\nDynasty/Maxstar 210/280, Syncrowave 300 Process And User Interface:\nBit / Error# / Description\n0 / 0.3.1 / Secondary Over Temp\n1 / 0.3.2 / Ambient Over Temp\n2 / 7.3.6 / Process Serial Communication With Gateway\n3 / 3.3.1 / Secondary Thermistor Failure\n4 / 3.3.2 / Ambient Thermistor Failure\n5 / 1.3.1 / Fan Failure\n6 / 1.3.2 / Clamp/Output Over Voltage\n7 / 1.3.3 / AC Commutation Time Out\n8 / 1.3.4 / Output Over Voltage\n9 / 1.3.5 / Output Current Or Voltage Feedback With Output Off\n10 / 1.3.6 / No Cooler Detected With Output Current\n11 / 7.3.4 / Process Serial Communication With Primary\n12 / 7.3.2 / Process Serial Communication With User Interface\n13 / 7.3.1 / Process Serial Communication With Memory Card\n14 / 7.3.5 / Process Serial Communication With CPS\n15 / 7.2.3 / User Interface Serial Communication With Process\nDynasty/Maxstar 400/800:\nBit / Error# / Description\n0 / 0.3.2 / Ambient Over Temp\n1 / 0.3.1 / Secondary Over Temp RC20\n2 / 0.3.1 / Secondary Over Temp RC30\n3 / 0.4.1 / Primary Power Over Temp 400/800 Top\n4 / 0.4.2 or 0.7.1 / Primary Power Over Temp 800 Bottom\n5\n6\n7\n8\n9\n10\n11 / 7.3.7 / Process serial communication with Primary 800 Bottom.\n12 / 7.3.4 / Process serial communication with Primary 400/800 Top.\n13 / 3.3.2 / Ambient thermistor failure\n14 / 3.3.1 / Secondary thermistor failure RC20\n15 / 3.3.1 / Secondary thermistor failure RC30",
};

pub const ERROR_REG_2: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4102 },
    name: "ERROR_REG_2",
    description: "Errors2, 16(Bits) Possible Errors, 1 True / 0 False (Power Source Dependent)\nDynasty/Maxstar 210/280, Syncrowave 300 Primary\nBit / Error# / Description\n0 / 0.4.1 / Primary Power 1 Over Temp\n1 / 0.4.2 / Primary Power 2 Over Temp\n2 / 1.4.8 / Ground Current\n3 / 1.4.0 / Primary Not Ready\n4 / 1.4.1 / Primary Capacitor Imbalance\n5 / 1.4.2 / Input Over Voltage\n6 / 1.4.3 / Input Over Current\n7 / 1.4.4 / Primary Bus Under Voltage\n8 / 1.4.5 / Input Under Voltage\n9 / 3.4.1 / Primary Power 1 Thermistor Failure\n10 / 3.4.2 / Primary Power 2 Thermistor Failure\n11 / 7.4.3 / Primary Serial Communication With Process\n12 / 1.4.6 / Primary Capacitor Failure\n13 / 1.4.7 / Primary Control Power\n14 / 0.4.1L / Primary Power 1 Latched Over Temp\n15 / 0.4.2L / Primary Power 2 Latched Over Temp\nDynasty/Maxstar 400/800, Syncrowave 400:\nBit / Error# / Description\n0 / 3.4.1 / Primary Power Thermistor Failure 400/800 Top\n1 / 3.4.2 or 3.7.1 / Primary Power Thermistor Failure 800 Bottom\n2 / 1.3.2 / Clamp/Output over voltage\n3 / 1.3.3 / AC Communication time out\n4 / 1.3.4 / Output over voltage\n5 / 1.3.5 / Output current or voltage feedback with output off\n6 / 1.4.8 / Ground current\n7 / 1.4.3 / Input over current 400/800 Top\n8 / 1.4.3 or 1.7.3 / Input over current 800 Bottom\n9 / 1.4.7 / Primary control power\n10 / 1.4.5 / Input under voltage\n11 / 1.4.4 / Primary bus under voltage\n12 / 7.3.6 / Process serial communication with Gateway\n13 / 7.3.2 / Process serial communication with User Interface\n14 / 7.3.1 / Process serial communication with Memory Card\n15 / 7.2.3 / User interface serial communication with Process",
};

pub const ERROR_REG_3: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4103 },
    name: "ERROR_REG_3",
    description: "Errors3, 16(Bits) Possible Errors, 1 True / 0 False (Power Source Dependent)\nDynasty Maxstar 210 And 280 CPS\nBit / Error# / Description\n0 / 0.5.1 / CPS Power Module 1 Over Temp\n1 / 0.5.2 / CPS Power Module 2 Over Temp\n2 / 0.5.3 / CPS Power Module 3 Over Temp\n3 / 1.5.9 / CPS Primary Bus Under Voltage\n4 / 7.5.3 / CPS Serial Communication With Process\n5 / 3.5.1 / CPS Power Module 1 Thermistor Failure\n6 / 3.5.2 / CPS Power Module 2 Thermistor Failure\n7 / 3.5.3 / CPS Power Module 3 Thermistor Failure\n8 / 1.5.1 / CPS Secondary Bus Under Voltage\n9 / 1.5.2 / CPS Output Over Current\n10 / 1.5.3 / CPS Secondary Bus Over Voltage\n11 / 1.5.4 / CPS Current Or Voltage feedback With CPS off\n12 / 1.5.5 / CPS Secondary Control Power\n13 / 1.5.6 / CPS Capacitor Imbalance\n14 / 1.5.7 / CPS Primary Control Power\n15 / 1.5.8 / CPS Secondary Communication With CPS Primary\nSyncrowave 300:\nBit/Error#/Description\n3/1.5.9/CPS Primary Bus Under Voltage\nDynasty/Maxstar 400/800, Syncrowave 400:\nBit/Error#/Description\n0 / 1.5.9 / CPS Primary Bus Under Voltage\n1 / 1.4.4 / Primary Bus Under Voltage 400/800 Top\n2 / 1.4.5 / Input Under Voltage 400/800 Top\n3 / 1.4.2 / Input Over Voltage 400/800 Top\n4 / 1.4.7 / Primary Control Power 400/800 Top\n5 / 7.4.3 / Primary Serial Communication With Process 400/800 Top\n6 / 1.4.0 / Primary Not Ready 400/800 Top\n7 /\n8 /\n9 / 1.7.4 / Primary Bus Under Voltage 800 Bottom\n10 / 1.7.5 / Input Under Voltage 800 Bottom\n11 / 1.7.2 / Input Over Voltage 800 Bottom\n12 / 1.7.7 / Primary Control Power 800 Bottom\n13 / 7.7.3 / Primary Serial Communication With Process 800 Bottom\n14 / 1.7.0 / Primary Not Ready 800 Bottom\n15 /",
};

pub const COMMANDED_OUTPUT_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4200 },
    name: "COMMANDED_OUTPUT_AMPERAGE",
    description: "Power Source Command Out Amperage, Res: 1A",
};

pub const OUTPUT_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4201 },
    name: "OUTPUT_AMPERAGE",
    description: "Power Source Output Current, Res: 1A",
};

pub const OUTPUT_VOLTAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4202 },
    name: "OUTPUT_VOLTAGE",
    description: "Power Source Output Voltage, Res: 0.1V",
};

pub const OUTPUT_CURRENT_DC_PULSE_PEAK: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4203 },
    name: "OUTPUT_CURRENT_DC_PULSE_PEAK",
    description: "Power Source Output Current DC Pulse Peak, Res: 1A",
};

pub const OUTPUT_VOLTAGE_DC_PULSE_PEAK: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4204 },
    name: "OUTPUT_VOLTAGE_DC_PULSE_PEAK",
    description: "Power Source Output Voltage DC Pulse Peak, Res 0.1V",
};

pub const OUTPUT_CURRENT_DC_PULSE_BACK: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4205 },
    name: "OUTPUT_CURRENT_DC_PULSE_BACK",
    description: "Power Source Output Current DC Pulse Back, Res: 1A",
};

pub const OUTPUT_VOLTAGE_DC_PULSE_BACK: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4206 },
    name: "OUTPUT_VOLTAGE_DC_PULSE_BACK",
    description: "Power Source Output Voltage DC Pulse Back, Res 0.1V",
};

pub const FAN_OUTPUT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4300 },
    name: "FAN_OUTPUT",
    description: "Fan Out, 0(Off) - 100%",
};

pub const TEMPERATURE_1: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4301 },
    name: "TEMPERATURE_1",
    description: "Temperature registers (Power Source Dependent):\nRange: 0 - 254,\nResolution: 1 Celsius\nOffset: -50 (i.e. 50 == 0 Deg. Celsius)\nPossible Range: -50 - +204 C\nActual Range: Limited By Thermistor’s Hardware And Software\n\nTemperature 1 (Dynasty/Maxstar 210/280, Syncrowave 300 - Primary Power 1)\n(Dynasty/Maxstar 400/800, Syncrowave 400 - Ambient)",
};

pub const TEMPERATURE_2: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4302 },
    name: "TEMPERATURE_2",
    description: "Temperature registers (Power Source Dependent):\nRange: 0 - 254,\nResolution: 1 Celsius\nOffset: -50 (i.e. 50 == 0 Deg. Celsius)\nPossible Range: -50 - +204 C\nActual Range: Limited By Thermistor’s Hardware And Software\n\nTemperature 2 (Dynasty/Maxstar 210/280, Syncrowave 300 - Primary Power 2) (Dynasty/Maxstar 400/800 Top, Syncrowave 400 - Primary Power)",
};

pub const TEMPERATURE_3: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4303 },
    name: "TEMPERATURE_3",
    description: "Temperature registers (Power Source Dependent):\nRange: 0 - 254,\nResolution: 1 Celsius\nOffset: -50 (i.e. 50 == 0 Deg. Celsius)\nPossible Range: -50 - +204 C\nActual Range: Limited By Thermistor’s Hardware And Software\n\nTemperature 3 (Dynasty/Maxstar 210/280, Syncrowave 300 - Secondary) (Dynasty/Maxstar 800 Bottom - Primary Power)",
};

pub const TEMPERATURE_4: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4304 },
    name: "TEMPERATURE_4",
    description: "Temperature registers (Power Source Dependent):\nRange: 0 - 254,\nResolution: 1 Celsius\nOffset: -50 (i.e. 50 == 0 Deg. Celsius)\nPossible Range: -50 - +204 C\nActual Range: Limited By Thermistor’s Hardware And Software\n\nTemperature 4 (Dynasty/Maxstar 210/280, Syncrowave 300 - Ambient) (Dynasty/Maxstar 400/800, Syncrowave 400 - Secondary RC20)",
};

pub const TEMPERATURE_5: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4305 },
    name: "TEMPERATURE_5",
    description: "Temperature registers (Power Source Dependent):\nRange: 0 - 254,\nResolution: 1 Celsius\nOffset: -50 (i.e. 50 == 0 Deg. Celsius)\nPossible Range: -50 - +204 C\nActual Range: Limited By Thermistor’s Hardware And Software\n\nTemperature 5 (Dynasty/Maxstar 210/280 - CPS Module 1) (Dynasty/Maxstar 400/800, Syncrowave 400 - Secondary RC30)",
};

pub const TEMPERATURE_6: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4306 },
    name: "TEMPERATURE_6",
    description: "Temperature registers (Power Source Dependent):\nRange: 0 - 254,\nResolution: 1 Celsius\nOffset: -50 (i.e. 50 == 0 Deg. Celsius)\nPossible Range: -50 - +204 C\nActual Range: Limited By Thermistor’s Hardware And Software\n\nTemperature 6 (Dynasty/Maxstar 210/280 - CPS Module 2)",
};

pub const TEMPERATURE_7: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4307 },
    name: "TEMPERATURE_7",
    description: "Temperature registers (Power Source Dependent):\nRange: 0 - 254,\nResolution: 1 Celsius\nOffset: -50 (i.e. 50 == 0 Deg. Celsius)\nPossible Range: -50 - +204 C\nActual Range: Limited By Thermistor’s Hardware And Software\n\nTemperature 7 (Dynasty/Maxstar 210/280 - CPS Module 3)",
};

pub const PRIMARY_LINE_CURRENT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4400 },
    name: "PRIMARY_LINE_CURRENT",
    description: "Dynasty/Maxstar 210/280, Syncrowave 300 - Primary Line Current, Res: 1A",
};

pub const PRIMARY_LINE_VOLTAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4401 },
    name: "PRIMARY_LINE_VOLTAGE",
    description: "Dynasty/Maxstar 210/280, Syncrowave 300 - Primary Line Voltage, Res: 1V",
};

pub const PRIMARY_LINE_VOLTAGE_PEAK: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4402 },
    name: "PRIMARY_LINE_VOLTAGE_PEAK",
    description: "Dynasty/Maxstar 210/280/400/800, Syncrowave 300/400 - Primary Line Voltage Peak, Res: 1V",
};

pub const PRIMARY_BUS_VOLTAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4403 },
    name: "PRIMARY_BUS_VOLTAGE",
    description: "Dynasty/Maxstar 210/280/400/800, Syncrowave 300/400 - Primary Bus Voltage, Res: 1V",
};

pub const COOLER_OUTPUT_VOLTAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4404 },
    name: "COOLER_OUTPUT_VOLTAGE",
    description: "Dynasty/Maxstar 210/280 - Cooler Power Output Voltage, Res: 1V",
};

pub const COOLER_OUTPUT_CURRENT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4405 },
    name: "COOLER_OUTPUT_CURRENT",
    description: "Dynasty/Maxstar 210/280 - Cooler Power Output Current, Res: 0.1A",
};

pub const COOLER_BUS_VOLTAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4406 },
    name: "COOLER_BUS_VOLTAGE",
    description: "Dynasty/Maxstar 210/280 - Cooler Power Bus Voltage, Res: 1V",
};

pub const PRIMARY_2_LINE_VOLTAGE_PEAK: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4407 },
    name: "PRIMARY_2_LINE_VOLTAGE_PEAK",
    description: "Dynasty/Maxstar 800 - Primary 2(bottom) Line Voltage Peak, Res: 1V",
};

pub const PRIMARY_2_BUS_VOLTAGE_PEAK: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::InputRegister, address: 4408 },
    name: "PRIMARY_2_BUS_VOLTAGE_PEAK",
    description: "Dynasty/Maxstar 800 - Primary 2(bottom) Bus Voltage Peak, Res: 1V",
};

pub const MB_SLAVE_ADDR: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6000 },
    name: "MB_SLAVE_ADDR",
    description: "Power Source’s Modbus Slave Address: 1 - 247.",
};

pub const FAN_REQUEST: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6001 },
    name: "FAN_REQUEST",
    description: "Fan Request:\nDynasty/Maxstar 210/280 0(Off), 1(Min 27%) - 30(Max 100%)\nRequires Request Of 3 Minimum To Start Fan\nDynasty/Maxstar 400/800 0(Off), 1 - 30(Max 100%)\nNotes: 1 second time out return to 0(Off).\nParallel Request With All Machine Thermistors, Where Highest Fan Request Is Used.\n0(Off) In This Register Will Not Turn Fan Off With A Fan Request Other Than Off.\nFrom Any Machine’s Thermistors.",
};

pub const CURRENT_METER_CALIBRATION: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6002 },
    name: "CURRENT_METER_CALIBRATION",
    description: "Meter Calibration, Amperage: +-50, Res: 0.1%, (+-50 == +-5.0%)Note: With Discrete Input 2011 RMS Hardware Detect = True, Coil 16 RMS Enable Selects RMS (True)Or Average (False) Amperage Calibration.",
};

pub const VOLTAGE_METER_CALIBRATION: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6003 },
    name: "VOLTAGE_METER_CALIBRATION",
    description: "Meter Calibration, Voltage Average: +-50, Res: 0.1%, (+-50 == +-5.0%)",
};

pub const ARC_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6100 },
    name: "ARC_TIME",
    description: "32 bit \n\nArc Time, Res: 0.01 Minute, Maximum: 59999999 == 9999 Hours And 59.99 Minutes.",
};

pub const ARC_CYCLES: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6102 },
    name: "ARC_CYCLES",
    description: "32 bit\n\nL Arc Cycles, Res: 1 Cycle, Maximum: 999999 Cycles.",
};

pub const MEMORY_CONTROL: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6200 },
    name: "MEMORY_CONTROL",
    description: "Dynasty/Maxstar 400/800 Models Only\nMemory: 0 Memory control off typically defaults to memory 1 with no memory number displayed.\n1 - Power Sources memory maximum",
};

pub const WELD_PROCESS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6201 },
    name: "WELD_PROCESS",
    description: "*Process: 0 Stick1 TIG2 MIG (Selectable only with Dynasty/Maxstar 210/280 Models and Dynasty’s Polarity DC)3 Test4 Hot Wire",
};

pub const WELD_START_PROCESS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6202 },
    name: "WELD_START_PROCESS",
    description: "*Process Start: 0 Scratch, 1 Lift, 2 HF.",
};

pub const TRIGGER: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6203 },
    name: "TRIGGER",
    description: "*Trigger: \n0 None-Output Off,\n1 Panel-Output ON\n2 Standard\n3 2T Hold\n4 3T Hold\n5 4T Hold\n6 4TL Mini Logic Hold\n7 4TE Momentary Hold\n8 4Tm Modified Hold",
};

pub const RMT_PIN_E_CONFIG: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6204 },
    name: "RMT_PIN_E_CONFIG",
    description: "*Remote 14-skt E Configuration:\n0 Amperage Control ( Slow Response, Finger Tip/Foot controls)\n1 External Pulse Control ( Amperage, Fast Response)\n2 Output Enable ( 14-Skt E-D Shorted Enables Power Source Output)\n3 Disable ( 14-Skt E Has No Function)",
};

pub const TUNGSTEN_PRESET: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6205 },
    name: "TUNGSTEN_PRESET",
    description: "*Tungsten (Canned Arc Start Parameters):\n0 0.020 in. (0.5mm)\n1 0.040 in. (1.0mm)\n2 1/16 in. (1.6mm)\n3 3/32 in. (2.4mm)\n4 1/8 in. (3.2mm)\n5 5/32 in. (4.0mm)\n6 3/16 in. (4.8mm)\n7 1/4 in. (6.4mm)\n8 General (User Defined With Holding Registers 6207 Through 6212)\n<9 Power Source Dependent, Typically Used With Process TIG\n9 Disabled (Typically Used With Non TIG Processes)",
};

pub const PRESET_MIN_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6206 },
    name: "PRESET_MIN_AMPERAGE",
    description: "Preset Amperage Minimum: Power Source AC / DC Amperage Minimum -25A(Tungsten General) Or 63A(Tungsten Disabled), Res 1A\nWrite Only With Tungsten General Or Disabled",
};

pub const ARC_START_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6207 },
    name: "ARC_START_AMPERAGE",
    description: "Arc Start Amperage: 5A - 200A, Res: 1AWrite Only With Tungsten General Or Disabled",
};

pub const ARC_START_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6208 },
    name: "ARC_START_TIME",
    description: "Arc Start Time: 0(Off) - 25(x10ms), Res: 1(x10ms)Write Only With Tungsten General",
};

pub const ARC_START_SLOPE_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6209 },
    name: "ARC_START_SLOPE_TIME",
    description: "Arc Start Slope Time: 0(Off) - 25(x10ms), Res: 1(x10ms)Write Only With Tungsten General",
};

pub const ARC_START_AC_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6210 },
    name: "ARC_START_AC_TIME",
    description: "**Arc Start AC Time: 0(Off) - 25(x10ms), Res: 1(x10ms)Write Only With AC Power Source’s AC Output And Tungsten General",
};

pub const ARC_START_POLARITY_PHASE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6211 },
    name: "ARC_START_POLARITY_PHASE",
    description: "**Arc Start Polarity Phase: 1 EP, 0 ENWrite Only With AC Power Source And Tungsten General or Disabled",
};

pub const AC_EN_WAVE_SHAPE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6212 },
    name: "AC_EN_WAVE_SHAPE",
    description: "*,**AC EN Wave Shape, 0 Advance Square, 1 Soft Square, 2 Sine, 3 Triangle",
};

pub const AC_EP_WAVE_SHAPE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6213 },
    name: "AC_EP_WAVE_SHAPE",
    description: "*,**AC EP Wave Shape, 0 Advance Square, 1 Soft Square, 2 Sine, 3 Triangle",
};

pub const HOT_START_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6214 },
    name: "HOT_START_TIME",
    description: "Hot Start Time:\nRange: 0(Off) -20\nResolution: 0.1 Second\nHot Start Enable / Disabled with Coil 8 Hot Start Enable.\nStick only",
};

pub const REMOTE_HOLD: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6215 },
    name: "REMOTE_HOLD",
    description: "Remote Hold:\n0 / 2T\n1 / 3T\n2 / 4T\n3 / 4TL Mini Logic\n4 / 4TE Momentary\n5 / 4Tm Modified\nResolution: 0.1 Second\nRemote Hold can also be changed with Holding Register 6204 Trigger.",
};

pub const DIG_PERCENT: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6217 },
    name: "DIG_PERCENT",
    description: "*Dig, 0(Off) - 100%, Res: 1%\n101% will set Process Stick for Carbon Arc Gouging, turning Dig off and disabling\nBoost (Coil 0009).\nWith Processes (Holding Register 6201) MIG selection:\n*Inductance 0 - 99% Res: 1%\n100% will set Inductance and optimize Digital Voltage Control for Flux Core Wire.",
};

pub const AC_EN_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6300 },
    name: "AC_EN_AMPERAGE",
    description: "*,**,***AC EN Amperage, Preset Amps Min - PS Amps Max, Res: 1A",
};

pub const AC_EP_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6301 },
    name: "AC_EP_AMPERAGE",
    description: "*,**,***AC EP Amperage, Preset Amps Min - PS Amps Max, Res: 1A",
};

pub const AC_BALANCE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6302 },
    name: "AC_BALANCE",
    description: "*,**,***AC Balance, 30-99%, Res: 1%",
};

pub const AC_FREQUENCY: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6303 },
    name: "AC_FREQUENCY",
    description: "*,**AC Frequency, 20-400Hz, Res: 1Hz",
};

pub const WELD_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6304 },
    name: "WELD_AMPERAGE",
    description: "*,***Weld Amperage(DC or AC), Preset Amps Min - PS Amps Max, Res: 1A",
};

pub const PULSER_PPS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6305 },
    name: "PULSER_PPS",
    description: "*Pulser - Pulses Per Second (PPS)\nRange: 0(Off) – 50000 / 5000 Power Source Dependent,\nResolution: 0.1 Hertz\nCan be set to a default value when writing a TRUE to coil 18 Pulser Enable and PPS is found at 0(Off).\nWriting a non “0” value will set coil 18 Pulser Enable to TRUE.\nWriting a “0” value will set coil 18 Pulser Enable to FALSE.\nDependent on configuration of the slave, the slave may or may not retain the PPS non “0” value.",
};

pub const PULSER_PEAK_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6306 },
    name: "PULSER_PEAK_TIME",
    description: "*Pulser - Peak Time, 5-95%, Res: 1%",
};

pub const PULSER_BACKGROUND_AMPS: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6307 },
    name: "PULSER_BACKGROUND_AMPS",
    description: "*Pulser - Background Amperage, 5-95%, Res: 1%",
};

pub const PRELOW_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6308 },
    name: "PRELOW_TIME",
    description: "*Prelow Time, 0(Off) - 250, Res: 1(x0.1Sec)",
};

pub const INITIAL_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6309 },
    name: "INITIAL_AMPERAGE",
    description: "*Initial Amperage, Preset Amps Min - PS Amps Max, Res: 1A",
};

pub const INITIAL_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6310 },
    name: "INITIAL_TIME",
    description: "*Initial Time, 0(Off) - 250, Res: 1(x0.1Sec)",
};

pub const INITIAL_SLOPE_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6311 },
    name: "INITIAL_SLOPE_TIME",
    description: "*Initial Slope Time, 0(Off) - 500, Res: 1(x0.1Sec)",
};

pub const MAIN_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6312 },
    name: "MAIN_TIME",
    description: "*Main Time, 0(Off) - 9990, Res: 1(x0.1Sec)",
};

pub const FINAL_SLOPE_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6313 },
    name: "FINAL_SLOPE_TIME",
    description: "*Final Slope Time, 0(Off) - 500, Res: 1(x0.1Sec)",
};

pub const FINAL_AMPERAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6314 },
    name: "FINAL_AMPERAGE",
    description: "*Final Amperage, Preset Amps Min - PS Amps Max, Res: 1A",
};

pub const FINAL_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6315 },
    name: "FINAL_TIME",
    description: "*Final Time, 0(Off) - 250, Res: 1(x0.1Sec)",
};

pub const POSTFLOW_TIME: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6316 },
    name: "POSTFLOW_TIME",
    description: "*Postflow Time, 0(Off) - 50S & Auto(51), Res: 1Sec",
};

pub const DIG_PERCENT_AGAIN: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6317 },
    name: "DIG_PERCENT_AGAIN",
    description: "*Dig, 0(Off) - 100%, Res: 1%101% will set Process Stick for Carbon Arc Gouging, turning Dig off and disabling Boost (Coil 0009).\n\nSame as 6217??",
};

pub const HOT_WIRE_VOLTAGE: RegisterMetadata = RegisterMetadata {
    address: RegisterAddress { register_type: ModbusAddressType::HoldingRegister, address: 6318 },
    name: "HOT_WIRE_VOLTAGE",
    description: "*Hot Wire Voltage, 5-20, Res: 1V",
};

pub const MILLER_REGISTERS: &[RegisterMetadata] = &[
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
    PRELOW_TIME,
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


