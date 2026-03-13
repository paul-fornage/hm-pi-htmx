## 9. Modbus Implemented Function Codes

### Table 5-1. Modbus Function Codes

| Function | Code |
| :--- | :--- |
| Read Coils | 1 |
| Read Discrete Inputs | 2 |
| Read Holding Registers | 3 |
| Read Input Registers | 4 |
| Write Single Coil | 5 |
| Write Single Register | 6 |
| Write Multiple Coils | 15 |
| Write Multiple Holding Registers | 16 |

## 10. Modbus Implemented Exception Codes

### Table 6-1. Modbus Exception Codes

| Exception | Code |
| :--- | :--- |
| Illegal Function | 01 |
| Illegal Data Address | 02 |
| Illegal Data Value | 03 |
| Server Device Failure | 04 |
| Server Device Busy | 06 |

## 11. Modbus Coils, Discrete Inputs, Input Registers, Holding Registers

**Notes:**
* See Dynasty Maxstar Owner's Manual for further understanding of functions controlled by most Modbus Coils, Discrete Inputs, Input Registers and Holding Registers.
* Input and Holding Registers with **L (Low)** and **H (High)** indicate two 16 bit registers combined to form 32 bit values.
* Read both L/H paired Input or Holding Registers at the same time to insure valid data values.
* Write L/H paired Holding Registers with function code "16 - Write Multiple Holding Registers" with address range including both registers. Failure to do so will result in exception response ILLEGAL DATA VALUE.
* Coil, User Interface Disable, may need to be set True to allow "*" marked Coils and Holding Registers to be set without User Interface interference.
* With User Interface disabled, all "*" marked Coils and Holding Registers should be set for desired function.
* AC capable (Dynasty) power source only.
* \*\*\* "AC Weld Amperage" (Aw), "AC EN Amperage" (Aen), "AC EP Amperage" (Aep) and "AC Balance" (%bal = % of "AC EN Amperage") are linked together where:
    * Setting "Aen", "Aep" or "%bal", will set "Aw" with: `$Aw=((Aen*\%bal)+(Aep*(1-\%bal)))$`
    * Ratio of "Aen" to "Aep" will be stored to be referenced when "Aw" is set.
* When setting "Aw":
    * "Aen" and "Aep" will track their last stored ratio while adjusting "Aw".
    * "Aw" will effectively be held to limits greater than "Preset Amps Min" or less than "PS Amps Max" when "Aen" or "Aep" reaches either "Preset Amps Min" or "PS Amps Max".
* With "Aen" and "Aep" set to the same value, "Aen" and "Aep" will track to the same value set in "Aw".

OM-265415 Page 4

---

### Table 7-1. Modbus Coils

| PDU Address | Coil | Name / Description / Resolution |
| :--- | :--- | :--- |
| 0000 | 0001 | \***User Interface Disable:** 1 True / 0 False. |
| 0001 | 0002 | **Remote Trigger (14-Skt B/Contactor) Disable:** 1 True / 0 False. |
| 0002 | 0003 | **Trigger (Contactor) Request:** 1 True (1 Second Time Out Return To False) / 0 False. To continue a weld sequence through Final Slope and or Final Time, Coil must be refreshed with False throughout these sequences. |
| 0003 | 0004 | **Gas Request:** 1 TRUE (1 Second Time Out Return To False) / 0 False. |
| 0004 | 0005 | \*\*\***AC Power Source's Output DC:** 1 True (DC) / 0 False (AC). |
| 0005 | 0006 | **AC Power Source's DC Polarity EP:** 1 True (EP) / 0 False (EN). |
| 0006 | 0007 | \***Stuck Check Enable:** 1 True / 0 False. |
| 0007 | 0008 | **Hot Start Enable:** 1 True/0 False.<br>*Note: Hot Start can also be Disabled with 0 time set in Holding Register 6215 Hot Start Time* |
| 0008 | 0009 | **Boost Enable:** 1 True/0 False. |
| 0009 | 0010 | **Droop Enable:** 1 True/0 False. |
| 0010 | 0011 | **Open Circuit Voltage (OCV) Low Enable:** 1 True (Low) / 0 False (Normal).<br>*OCV selection applies to both Stick and MIG processes.* |
| 0011 | 0012 | \***Weld Gas Enable:** 1 True/0 False Enables Gas With Contactor.<br>**CE Models Only** |
| 0012 | 0013 | **Cooler Power Supply (CPS) Enable:** 1 True (Parallel With Coil 0014) / 0 False.<br>*Note: Dynasty/Maxstar 210/280 CE Models Have No Control, Read Returns False.* |
| 0013 | 0014 | \***Cooler Power Supply (CPS) TIG Enable:** 1 True (Parallel With Coil 0013) / 0 False TIG Process Control Of Cooler Power Supply.<br>**Dynasty/Maxstar 210/280 Models Only** |
| 0014 | 0015 | \***Cooler Error Enable:** 1 True/0 False Enables Error "1.3.6 No Cooler Detected With Output Current". Error Is Generated When No Load Detected On Cooler Power Supply's Output With Load Detected On The Power Source's Output. |
| 0015 | 0016 | **Touch Sense Enable:** 1 True / 0 False.<br>Touch Sense Detection found at Modbus Discrete Input 2009 Or Remote 14 Receptacle Socket J. |
| 0016 | 0017 | **RMS Enable:** AC Amperage Preset And Meter And/Or DC Pulse Amperage Meter: 1 True (RMS) / 0 False (Average)<br>*Note: To Enable, Must Have Discrete Input 2013 RMS Hardware Detect = True.* |
| 0017 | 0018 | \***Pulser Enable:** 1 True/0 False.<br>*Note: Can also be set TRUE / FALSE when writing values to Holding Register 6305 Pulser Pulses Per Second (PPS). When enabled and Holding Register 6305 PPS is found at "0", PPS will be set to a default value.* |
| 0018 | 0019 | **Dynasty/Maxstar 400/800 Models Only**<br>\***AC Commutation Amperage LOW ENABLE:** 1 TRUE (LOW)/0 FALSE (High) |
| 0019 | 0020 | \***AC Independent Enable:** 1 True/0 False. Enables/Disables Both Independent Amperage and Independent AC Wave Shapes. |
| 0020 | 0021 | \***Weld Timers Enable:** 1 True/0 False. Weld Timers Include Weld (Spot), Intial Amperage and Final Amperage Timers. |

OM-265415 Page 5

---

### Table 7-2. Modbus Discrete Inputs

| PDU Address | Discrete Input | Name / Description / Resolution |
| :--- | :--- | :--- |
| 2000 | 2001 | **Dynasty/Maxstar 210/280 Models Only**<br>Cooler Power Supply (CPS) Detect: 1 True/0 False. |
| 2001 | 2002 | **Dynasty/Maxstar 210/280 Models Only**<br>Cooler Load Detect: 1 True/0 False. |
| 2002 | 2003 | **Foot/Finger Tip Control Detect:** 1 True/0 False<br>*Note: Holding Register 6205 (Remote 14-Skt E) Must Be Configured To 0 (Amperage Control) To Detect Foot/Finger Tip Control.* |
| 2003 | 2004 | **Remote Trigger (Contactor 14-Skt A-B) Enable:** 1 True / 0 False. |
| 2004 | 2005 | **Contactor Output Enabled:** 1 True/0 False (Contactor Output Or Sense Voltage Pre Contactor Output). |
| 2005 | 2006 | **Gas Output Enabled:** 1 True / 0 False. |
| 2006 | 2007 | **Valid Arc:** 1 True/0 False. |
| 2007 | 2008 | **Arc Length Control Lock Out:** 1 True/0 False. |
| 2008 | 2009 | **Touch Sense Detect:** 1 True / 0 False. Touch Sense Enable (Coil 16) Must Be Set True With Machine's State (Input Register 4101) In Standby, And Weld Output Shorted For Touch Sense Detect To Register As True. |
| 2009 | 2010 | **CE Model Detect:** 1 True / 0 False |
| 2010 | 2011 | **STR Model Detect:** 1 True/0 False |
| 2011 | 2012 | **DX Model Detect:** 1 True / 0 False |
| 2012 | 2013 | **RMS Hardware Detect:** 1 True/0 False |
| 2013 | 2014 | **Low Line Detect:** 1 True/0 False (**Dynasty/Maxstar 210 Only**)<br>*Note: Set True When Powered Up On 120 V Input.* |
| 2014 | 2015 | **Feature Enable for Hot Start Adjust:** 1 True/0 False. |
| 2015 | 2016 | **Feature Enable for AC Independent:** 1 True/0 False. |
| 2016 | 2017 | **Dynasty/Maxstar 210/280 Models Only**<br>Volt Sensing (MIG) Model Detect: 1 True/0 False |
| 2017 | 2018 | **Syncrowave Model Detect:** 1 True/0 False |
| 2018 | 2019 | **Syncrowave 300/400 Models Only**<br>Non Cooler Supply Detect: 1 True/0 False |

OM-265415 Page 6

---

### Table 7-3. Modbus Input Registers

| PDU Address | Input Registers | Name / Description / Resolution |
| :--- | :--- | :--- |
| 4016<br>4017 | 4017 L<br>4018 H | **Dynasty/Maxstar 800 Models Only**<br>**Application Software Number And Revision**, 4 Bytes Bit Mapped:<br>`NNNNNNNN NNNN, NNNN NNNN,NNRR RRRE,EEEE`<br><ul><li>`NNNNNNNN NNNNNNNN NNNN.NN` == Miller Part Number, 22 Bits 31-10, Bit Range 0-4,194,303, Actual 0-999999</li><li>`RR RRR` = Revision Level, 5 Bits 9-5, Bit Range 0-31, Actual 0-26 where: `0` == "0" Preproduction Or Field Test Software 1,2,3... == Revision A,B,C...</li><li>`E,EEEE` == Evaluation / Test, 5 Bits 9-5, Bit Range 0-31, Actual 0-26 Where: `0` == "6" Released Software, 1,2,3... Evaluation / Test Revision A,B,C...</li></ul>**PCB 7 Primary** |
| 4018<br>4019 | 4019 L<br>4020 H | **Application Software Number And Revision**, **PCB 6 Gateway Interface** |
| 4020<br>4021 | 4021 L<br>4022 H | **Application Software Number And Revision**, **Dynasty/Maxstar 210/280 Models Only**<br>**Application Software Number And Revision**, **PCB 5 Cooler Power Supply (CPS)** |
| 4022<br>4023 | 4023 L<br>4024 H | **Application Software Number And Revision**, **PCB 4 Primary** |
| 4024<br>4025 | 4025 L<br>4026 H | **Application Software Number And Revision**, **PCB 3 Process** |
| 4026<br>4027 | 4027 L<br>4028 H | **Application Software Number And Revision**, **PCB 2 User Interface** |
| 4028<br>4029 | 4029 L<br>4030 H | **Application Software Number And Revision**, **PCB 1 SD Card** |

OM-265415 Page 7

| PDU Address | Input Registers | Name / Description / Resolution |
| :--- | :--- | :--- |
| 4030<br>4031 | 4031 L<br>4032 H | **Serial Number:**<br>4 Bytes Bit Mapped:<br>`DDDY, YYYW WWWW.WSSS SSSS,SSSS SSSB, BBBB`<br><ul><li>`DDD` = Decade Code, 3 Bits 31-29, Bit Range 0-7, actual "M" - "U" (For Decades 201*-208*), Skip "O", See Note</li><li>`Y,YYY` = Year Code, 4 Bits 28-25, Bit Range 0-15, Actual 0-9 "A" - "K", Skip "I", See Note</li><li>`WWWW.W` = Week Number, 6 Bits 24-19, Bit Range 0-63, Actual 01-52</li><li>`SSS SSSS,SSSS SSS` = Serialized Number, 14 Bits 18-5, Bit Range 0-16383, Actual 0001-9999</li><li>`B,BBBB` = Business Unit Code, 5 Bits 4-0, Bit Range 0-31. Actual 0-25 "A"-"Z", "I" And "O", Not Used See Note</li></ul>*Note:*<br>*Letters "I" And "O", Similar To Numbers "1" And "0" Skipped In Decade And Year.*<br>*Not used In Business Unit Code.* |
| 4032 | 4033 | **Power Source Configuration**, Amperage Maximum: 0-1023, Res: 1A |
| 4033 | 4034 | **Power Source Configuration**, Amperage DC Minimum: 0-31, Res: 1A,<br>0 DC Not Available |
| 4034 | 4035 | **Power Source Configuration**, Amperage AC Minimum: 0-31, Res: 1A,<br>0= AC Not Available |
| 4036<br>4037 | 4037 L<br>4038 H | **Machine's Software Update Number, Revision.**<br>4 Bytes Bit Mapped:<br>`NNNN, NNNN NNNN, NNNN NNNN,NNMM MMML,LLLL`<br><ul><li>`NNNN, NNNN NNNN,NNNN NNNN, NN` = Miller Part Number, 22 Bits 31-10, Bit Range 0-4,194,303, Actual 0-999999</li><li>`MM MMM` = Revision Level's Most Significant Designator, 5 Bits 9-5, Bit Range 0-31, Actual 0,1-26 (ASCII "@,A-Z"), 9 "I" & 15 "O" Similar To "1" & "0" Not Used. Typically Starts At 0 ("@", Omitted When Displayed), Increases By One With Each Wrap "Z" To "A" Of The Least Significant Designator</li><li>`L, LLLL` = Revision Level's Least Significant Designator, 5 Bits 4-0, Bit Range 0-31, Actual 0,1-26 (ASCII "@,A-Z"), 9 "I" & 15 "O" Similar To "1" & "0" Not Used. 0 "@" Used For Preproduction Only.</li></ul> |

OM-265415 Page 8

| PDU Address | Input Registers | Name / Description / Resolution |
| :--- | :--- | :--- |
| 4099 | 4100 | **Sequence Timer:**<br>**Remaining/Elapsed Time of States:**<br><ul><li>Initial Amperage</li><li>Initial Slope Time</li><li>Main Amperage</li><li>Final Slope Time</li><li>Final Amperage</li><li>Preflow</li><li>Postflow (typically timed while in Standby State)</li></ul>**Resolution:** 0.1 Second |
| 4100 | 4101 | **State:**<br><ul><li>0 Initial Amperage</li><li>1 Initial Slope Time</li><li>2 Main Amperage</li><li>3 Final Slope Time</li><li>4 Final Amperage</li><li>5 Preflow</li><li>6 Standby</li><li>7 Output Shorted</li><li>8 Release Trigger</li><li>9 Output Disabled</li><li>13 Error</li><li>14 Power Down</li><li>15 Power Up</li></ul> |
| 4101 | 4102 | **Errors1, 16(Bits) Possible Errors, 1 True/0 False (Power Source Dependent)**<br><br>**Dynasty/Maxstar 210/280, Syncrowave 300 Process And User Interface:**<br>**Bit/Error#/Description**<br><ul><li>`0` / `0.3.1` / Secondary Over Temp</li><li>`1` / `0.3.2` / Ambient Over Temp</li><li>`2` / `7.3.6` / Process Serial Communication With Gateway</li><li>`3` / `3.3.1` / Secondary Thermistor Failure</li><li>`4` / `3.3.2` / Ambient Thermistor Failure</li><li>`5` / `1.3.1` / Fan Failure</li><li>`6` / `1.3.2` / Clamp/Output Over Voltage</li><li>`7` / `1.3.3` / AC Commutation Time Out</li><li>`8` / `1.3.4` / Output Over Voltage</li><li>`9` / `1.3.5` / Output Current Or Voltage Feedback With Output Off</li><li>`10` / `1.3.6` / No Cooler Detected With Output Current</li><li>`11` / `7.3.4` / Process Serial Communication With Primary</li><li>`12` / `7.3.2` / Process Serial Communication With User Interface</li><li>`13` / `7.3.1` / Process Serial Communication With Memory Card</li><li>`14` / `7.3.5` / Process Serial Communication With CPS</li><li>`15` / `7.2.3` / User Interface Serial Communication With Process</li></ul><br>**Dynasty/Maxstar 400/800:**<br>**Bit/Error#/Description**<br><ul><li>`0` / `0.3.2` / Ambient Over Temp</li><li>`1` / `0.3.1` / Secondary Over Temp RC20</li><li>`2` / `0.3.1` / Secondary Over Temp RC30</li><li>`3` / `0.4.1` / Primary Power Over Temp 400/800 Top</li><li>`4` / `0.4.2 or 0.7.1` / Primary Power Over Temp 800 Bottom</li><li>`5` </li><li>`6` </li><li>`7` </li><li>`8` </li><li>`9` </li><li>`10` </li><li>`11` / `7.3.7` / Process serial communication with Primary 800 Bottom.</li><li>`12` / `7.3.4` / Process serial communication with Primary 400/800 Top.</li><li>`13` / `3.3.2` / Ambient thermistor failure</li><li>`14` / `3.3.1` / Secondary thermistor failure RC20</li><li>`15` / `3.3.1` / Secondary thermistor failure RC30</li></ul> |

OM-265415 Page 9

| PDU Address | Input Registers | Name / Description / Resolution |
| :--- | :--- | :--- |
| 4102 | 4103 | **Errors2, 16(Bits) Possible Errors, 1 True/0 False (Power Source Dependent)**<br><br>**Dynasty/Maxstar 210/280, Syncrowave 300 Primary:**<br>**Bit/Error#/Description**<br><ul><li>`0` / `0.4.1` / Primary Power 1 Over Temp</li><li>`1` / `0.4.2` / Primary Power 2 Over Temp</li><li>`2` / `1.4.8` / Ground Current</li><li>`3` / `1.4.0` / Primary Not Ready</li><li>`4` / `1.4.1` / Primary Capacitor Imbalance</li><li>`5` / `1.4.2` / Input Over Voltage</li><li>`6` / `1.4.3` / Input Over Current</li><li>`7` / `1.4.4` / Primary Bus Under Voltage</li><li>`8` / `1.4.5` / Input Under Voltage</li><li>`9` / `3.4.1` / Primary Power 1 Thermistor Failure</li><li>`10` / `3.4.2` / Primary Power 2 Thermistor Failure</li><li>`11` / `7.4.3` / Primary Serial Communication With Process</li><li>`12` / `1.4.6` / Primary Capacitor Failure</li><li>`13` / `1.4.7` / Primary Control Power</li><li>`14` / `0.4.1L` / Primary Power 1 Latched Over Temp</li><li>`15` / `0.4.2L` / Primary Power 2 Latched Over Temp</li></ul><br>**Dynasty/Maxstar 400/800, Syncrowave 400:**<br>**Bit/Error#/Description**<br><ul><li>`0` / `3.4.1` / Primary Power Thermistor Failure 400/800 Top</li><li>`1` / `3.4.2 or 3.7.1` / Primary Power Thermistor Failure 800 Bottom</li><li>`2` / `1.3.2` / Clamp/Output over voltage</li><li>`3` / `1.3.3` / AC Communication time out</li><li>`4` / `1.3.4` / Output over voltage</li><li>`5` / `1.3.5` / Output current or voltage feedback with output off</li><li>`6` / `1.4.8` / Ground current</li><li>`7` / `1.4.3` / Input over current 400/800 Top</li><li>`8` / `1.4.3 or 1.7.3` / Input over current 800 Bottom</li><li>`9` / `1.4.7` / Primary control power</li><li>`10` / `1.4.5` / Input under voltage</li><li>`11` / `1.4.4` / Primary bus under voltage</li><li>`12` / `7.3.6` / Process serial communication with Gateway</li><li>`13` / `7.3.2` / Process serial communication with User Interface</li><li>`14` / `7.3.1` / Process serial communication with Memory Card</li><li>`15` / `7.2.3` / User interface serial communication with Process</li></ul> |

OM-265415 Page 10

| PDU Address | Input Registers | Name / Description / Resolution |
| :--- | :--- | :--- |
| 4103 | 4104 | **Errors3, 16(Bits) Possible Errors, 1 True/0 False (Power Source Dependent)**<br><br>**Dynasty Maxstar 210 And 280 CPS**<br>**Bit/Error#/Description**<br><ul><li>`0` / `0.5.1` / CPS Power Module 1 Over Temp</li><li>`1` / `0.5.2` / CPS Power Module 2 Over Temp</li><li>`2` / `0.5.3` / CPS Power Module 3 Over Temp</li><li>`3` / `1.5.9` / CPS Primary Bus Under Voltage</li><li>`4` / `7.5.3` / CPS Serial Communication With Process</li><li>`5` / `3.5.1` / CPS Power Module 1 Thermistor Failure</li><li>`6` / `3.5.2` / CPS Power Module 2 Thermistor Failure</li><li>`7` / `3.5.3` / CPS Power Module 3 Thermistor Failure</li><li>`8` / `1.5.1` / CPS Secondary Bus Under Voltage</li><li>`9` / `1.5.2` / CPS Output Over Current</li><li>`10` / `1.5.3` / CPS Secondary Bus Over Voltage</li><li>`11` / `1.5.4` / CPS Current Or Voltage feedback With CPS off</li><li>`12` / `1.5.5` / CPS Secondary Control Power</li><li>`13` / `1.5.6` / CPS Capacitor Imbalance</li><li>`14` / `1.5.7` / CPS Primary Control Power</li><li>`15` / `1.5.8` / CPS Secondary Communication With CPS Primary</li></ul><br>**Syncrowave 300:**<br>**Bit/Error#/Description**<br><ul><li>`3` / `1.5.9` / CPS Primary Bus Under Voltage</li></ul><br>**Dynasty/Maxstar 400/800, Syncrowave 400:**<br>**Bit/Error#/Description**<br><ul><li>`0` / `1.5.9` / CPS Primary Bus Under Voltage</li><li>`1` / `1.4.4` / Primary Bus Under Voltage 400/800 Top</li><li>`2` / `1.4.5` / Input Under Voltage 400/800 Top</li><li>`3` / `1.4.2` / Input Over Voltage 400/800 Top</li><li>`4` / `1.4.7` / Primary Control Power 400/800 Top</li><li>`5` / `7.4.3` / Primary Serial Communication With Process 400/800 Top</li><li>`6` / `1.4.0` / Primary Not Ready 400/800 Top</li><li>`7` </li><li>`8` </li><li>`9` / `1.7.4` / Primary Bus Under Voltage 800 Bottom</li><li>`10` / `1.7.5` / Input Under Voltage 800 Bottom</li><li>`11` / `1.7.2` / Input Over Voltage 800 Bottom</li><li>`12` / `1.7.7` / Primary Control Power 800 Bottom</li><li>`13` / `7.7.3` / Primary Serial Communication With Process 800 Bottom</li><li>`14` / `1.7.0` / Primary Not Ready 800 Bottom</li><li>`15` </li></ul> |

OM-265415 Page 11

---

| PDU Address | Input Registers | Name / Description / Resolution |
| :--- | :--- | :--- |
| 4200 | 4201 | **Power Source Command Out Amperage**, Res: 1A |
| 4201 | 4202 | **Power Source Output Current**, Res: 1A |
| 4202 | 4203 | **Power Source Output Voltage**, Res: 0.1V |
| 4203 | 4204 | **Power Source Output Current DC Pulse Peak**, Res: 1A |
| 4204 | 4205 | **Power Source Output Voltage DC Pulse Peak**, Res 0.1V |
| 4205 | 4206 | **Power Source Output Current DC Pulse Back**, Res: 1A |
| 4206 | 4207 | **Power Source Output Voltage DC Pulse Back**, Res 0.1V |
| 4300 | 4301 | **Fan Out**, 0(Off) - 100% |
| 4301 | 4302 | **Temperature registers (Power Source Dependent):**<br>Range: 0-254,<br>Resolution: 1 Celsius<br>Offset: -50 (i.e. $50==0$ Deg. Celsius)<br>Possible Range: -50 - +204 C<br>Actual Range: Limited By Thermistor's Hardware And Software<br>**Temperature 1** (Dynasty/Maxstar 210/280, Syncrowave 300 - Primary Power 1)<br>(Dynasty/Maxstar 400/800, Syncrowave 400- Ambient) |
| 4302 | 4303 | **Temperature 2** (Dynasty/Maxstar 210/280, Syncrowave 300 - Primary Power 2)<br>(Dynasty/Maxstar 400/800 Top, Syncrowave 400 - Primary Power) |
| 4303 | 4304 | **Temperature 3** (Dynasty/Maxstar 210/280, Syncrowave 300 - Secondary)<br>(Dynasty/Maxstar 800 Bottom - Primary Power) |
| 4304 | 4305 | **Temperature 4** (Dynasty/Maxstar 210/280, Syncrowave 300 - Ambient)<br>(Dynasty/Maxstar 400/800, Syncrowave 400 - Secondary RC20) |
| 4305 | 4306 | **Temperature 5** (Dynasty/Maxstar 210/280 - CPS Module 1)<br>(Dynasty/Maxstar 400/800, Syncrowave 400 - Secondary RC30) |
| 4306 | 4307 | **Temperature 6** (Dynasty/Maxstar 210/280 - CPS Module 2) |
| 4307 | 4308 | **Temperature 7** (Dynasty/Maxstar 210/280 - CPS Module 3) |
| 4400 | 4401 | **Dynasty/Maxstar 210/280, Syncrowave 300** - Primary Line Current, Res: 1A |
| 4401 | 4402 | **Dynasty/Maxstar 210/280, Syncrowave 300** - Primary Line Voltage, Res: 1V |
| 4402 | 4403 | **Dynasty/Maxstar 210/280/400/800, Syncrowave 300/400** - Primary Line Voltage Peak, Res: 1V |
| 4403 | 4404 | **Dynasty/Maxstar 210/280/400/800, Syncrowave 300/400** - Primary Bus Voltage, Res: 1V |
| 4404 | 4405 | **Dynasty/Maxstar 210/280** - Cooler Power Output Voltage, Res: 1V |
| 4405 | 4406 | **Dynasty/Maxstar 210/280** - Cooler Power Output Current, Res: 0.1A |
| 4406 | 4407 | **Dynasty/Maxstar 210/280** - Cooler Power Bus Voltage, Res: 1V |
| 4407 | 4408 | **Dynasty/Maxstar 800** - Primary 2(bottom) Line Voltage Peak, Res: 1V |
| 4408 | 4409 | **Dynasty/Maxstar 800** - Primary 2(bottom) Bus Voltage Peak, Res: 1V |

OM-265415 Page 12

---

### Table 7-4. Modbus Holding Registers

| PDU Address | Holding Registers | Name / Description / Resolution |
| :--- | :--- | :--- |
| 6000 | 6001 | **Power Source's Modbus Slave Address:** 1-247. |
| 6001 | 6002 | **Fan Request:**<br><ul><li>**Dynasty/Maxstar 210/280**: 0(Off), 1 (Min 27%)-30(Max 100%) Requires Request Of 3 Minimum To Start Fan</li><li>**Dynasty/Maxstar 400/800**: 0(Off), 1-30(Max 100%)</li></ul>*Notes: 1 second time out return to 0(Off). Parallel Request With All Machine Thermistors, Where Highest Fan Request Is Used. 0(Off) In This Register Will Not Turn Fan Off With A Fan Request Other Than Off. From Any Machine's Thermistors.* |
| 6002 | 6003 | **Meter Calibration, Amperage:** +-50, Res: 0.1%, $(+-50==+-5.0\%)$<br>*Note: With Discrete Input 2012 RMS Hardware Detect = True, Coil 17 RMS Enable Selects RMS (True) Or Average (False) Amperage Calibration.* |
| 6003 | 6004 | **Meter Calibration, Voltage Average:** +-50, Res: 0.1%, $(+-50==+-5.0\%)$ |
| 6100<br>6101 | 6101 L<br>6102 H | **Arc Time**, Res: 0.01 Minute, Maximum: 59999999 == 9999 Hours And 59.99 Minutes. |
| 6102<br>6103 | 6103 L<br>6104 H | **Arc Cycles**, Res: 1 Cycle, Maximum: 999999 Cycles. |
| 6200 | 6201 | **Dynasty/Maxstar 400/800 Models Only**<br>**Memory:**<br><ul><li>0 Memory control off typically defaults to memory 1 with no memory number displayed.</li><li>1- Power Sources memory maximum</li></ul> |
| 6201 | 6202 | \***Process:**<br><ul><li>0 Stick</li><li>1 TIG</li><li>2 MIG (Selectable only with Dynasty/Maxstar 210/280 Models and Dynasty's Polarity DC)</li><li>3 Test</li><li>4 Hot Wire</li></ul> |
| 6202 | 6203 | \***Process Start:** 0 Scratch, 1 Lift, 2 HF. |
| 6203 | 6204 | \***Trigger:**<br><ul><li>0 None-Output Off,</li><li>1 Panel-Output ON</li><li>2 Standard</li><li>3 2T Hold</li><li>4 3T Hold</li><li>5 4T Hold</li><li>6 4TL Mini Logic Hold</li><li>7 4TE Momentary Hold</li><li>8 4Tm Modified Hold</li></ul> |
| 6204 | 6205 | \***Remote 14-skt E Configuration:**<br><ul><li>0 Amperage Control (Slow Response, Finger Tip/Foot controls)</li><li>1 External Pulse Control (Amperage, Fast Response)</li><li>2 Output Enable (14-Skt E-D Shorted Enables Power Source Output)</li><li>3 Disable (14-Skt E Has No Function)</li></ul> |
| 6205 | 6206 | \***Tungsten (Canned Arc Start Parameters):**<br><ul><li>0 0.020 in. (0.5mm)</li><li>1 0.040 in. (1.0mm)</li><li>2 1/16 in(1.6mm)</li><li>3 3/32 in. (2.4mm)</li><li>4 1/8 in. (3.2mm)</li><li>5 5/32 in. (4.0mm)</li><li>6 3/16 in. (4.8mm)</li><li>7 1/4 in. (6.4mm)</li><li>8 General (User Defined With Holding Registers 6207 Through 6212)</li><li>$<9$ Power Source Dependent, Typically Used With Process TIG</li><li>9 Disabled (Typically Used With Non TIG Processes)</li></ul> |
| 6206 | 6207 | **Preset Amperage Minimum:** Power Source AC/DC Amperage Minimum - 25A(Tungsten General) Or 63A (Tungsten Disabled), Res 1A<br>*Write Only With Tungsten General Or Disabled* |
| 6207 | 6208 | **Arc Start Amperage:** 5A-200A, Res: 1A<br>*Write Only With Tungsten General Or Disabled* |

OM-265415 Page 13

| PDU Address | Holding Registers | Name / Description / Resolution |
| :--- | :--- | :--- |
| 6208 | 6209 | **Arc Start Time:** 0(Off) - 25(x10ms), Res: 1(x10ms)<br>*Write Only With Tungsten General* |
| 6209 | 6210 | **Arc Start Slope Time:** 0(Off) - 25(x10ms), Res: 1(x10ms)<br>*Write Only With Tungsten General* |
| 6210 | 6211 | \*\***Arc Start AC Time:** 0(Off) -25(x10ms), Res: 1 (x10ms)<br>*Write Only With AC Power Source's AC Output And Tungsten General* |
| 6211 | 6212 | \*\*\***Arc Start Polarity Phase:** 1 EP, 0 EN<br>*Write Only With AC Power Source And Tungsten General or Disabled* |
| 6212 | 6213 | \*\*\***AC EN Wave Shape,** 0 Advance Square, 1 Soft Square, 2 Sine, 3 Triangle |
| 6213 | 6214 | \*\*\***AC EP Wave Shape,** 0 Advance Square, 1 Soft Square, 2 Sine, 3 Triangle |
| 6214 | 6215 | **Hot Start Time:**<br>Range: 0(Off) -20<br>Resolution: 0.1 Second<br>*Hot Start Enable / Disabled with Coil 8 Hot Start Enable.* |
| 6215 | 6216 | **Remote Hold:**<br><ul><li>$0/2T$</li><li>$1/3T$</li><li>$2/4T$</li><li>$3/4TL$ Mini Logic</li><li>$4/4TE$ Momentary</li><li>5/4Tm Modified</li></ul>Resolution: 0.1 Second<br>*Remote Hold can also be changed with Holding Register 6204 Trigger.* |
| 6217 | 6218 | \***Dig**, 0(Off) - 100%, Res: 1%<br>101% will set Process Stick for Carbon Arc Gouging, turning Dig off and disabling Boost (Coil 0009).<br><br>**With Processes (Holding Register 6201) MIG selection:**<br>\***Inductance** 0-99% Res: 1%<br>100% will set Inductance and optimize Digital Voltage Control for Flux Core Wire. |
| 6300 | 6301 | \*\*\*\*\*\***AC EN Amperage**, Preset Amps Min - PS Amps Max, Res: 1A |
| 6301 | 6302 | \*\*\*\*\*\***AC EP Amperage**, Preset Amps Min - PS Amps Max, Res: 1A |
| 6302 | 6303 | \*\*\*\*\*\***AC Balance**, 30-99%, Res: 1% |
| 6303 | 6304 | \*\*\***AC Frequency**, 20-400Hz, Res: 1Hz |
| 6304 | 6305 | \*\*\*\***Weld Amperage (DC or AC)**, Preset Amps Min - PS Amps Max, Res: 1A |
| 6305 | 6306 | Resolution: 0.1 Hertz<br>\***Pulser - Pulses Per Second (PPS)**<br>Range: 0(Off)-50000/5000 Power Source Dependent,<br>*Can be set to a default value when writing a TRUE to coil 18 Pulser Enable and PPS is found at 0(Off).*<br>*Writing a non "0" value will set coil 18 Pulser Enable to TRUE.*<br>*Writing a "0" value will set coil 18 Pulser Enable to FALSE.*<br>*Dependent on configuration of the slave, the slave may or may not retain the PPS non "0" value.* |
| 6306 | 6307 | \***Pulser - Peak Time**, 5-95%, Res: 1% |
| 6307 | 6308 | \***Pulser - Background Amperage**, 5-95%, Res: 1% |
| 6308 | 6309 | \***Preflow Time**, 0(Off) - 250, Res: 1(x0.1Sec) |
| 6309 | 6310 | \***Initial Amperage**, Preset Amps Min - PS Amps Max, Res: 1A |
| 6310 | 6311 | \***Initial Time**, 0(Off) - 250, Res: 1 (x0.1Sec) |
| 6311 | 6312 | \***Initial Slope Time**, 0(Off) -500, Res: 1(x0.1Sec) |
| 6312 | 6313 | \***Main Time**, 0(Off) - 9990, Res: 1(x0.1Sec) |
| 6313 | 6314 | \***Final Slope Time**, 0(Off) -500, Res: 1(x0.1Sec) |
| 6314 | 6315 | \***Final Amperage**, Preset Amps Min - PS Amps Max, Res: 1A |
| 6315 | 6316 | \***Final Time**, 0(Off) - 250, Res: 1(x0.1Sec) |
| 6316 | 6317 | \***Postflow Time**, 0(Off) 50S & Auto(51), Res: 1Sec |
| 6317 | 6318 | \***Dig**, 0(Off)-100%, Res: 1%<br>101% will set Process Stick for Carbon Arc Gouging, turning Dig off and disabling Boost (Coil 0009). |
| 6318 | 6319 | \***Hot Wire Voltage**, 5-20, Res: 1V |

OM-265415 Page 14