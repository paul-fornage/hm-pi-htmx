

`tailwindcss -i styles/input.css -o static/assets/main.css`



TODO: 
- [ ] preemptive frontend feedback for things like mandrel must be closed or fingers not down
- [ ] jog buttons on touch screen don't work
- [ ] text inputs with numeric min and max and step
- [ ] `autocomplete="off"` for inputs - maybe combine with above for a custom input component?
- [ ] When overwriting existing profiles, it should not use the browser pop up. It should use the same idea as the delete button in the list of 'load' profiles
- [ ] UDP port in CC static config was moved.
- [ ] trailing hover effect in header. Remove all hover: selectors?
- [ ] manual control should not show disabled axes


- new addresses:
  - Cycle
    - HregAddr::CYCLE_START_POS
    - HregAddr::CYCLE_END_POS
    - HregAddr::CYCLE_PARK_POS
    - HregAddr::CYCLE_WELD_SPEED
    - HregAddr::CYCLE_REPOSITION_SPEED_X
    - HregAddr::CYCLE_REPOSITION_SPEED_Y
    - HregAddr::CYCLE_REPOSITION_SPEED_Z
    - HregAddr::CYCLE_WIRE_FEED_SPEED
    - HregAddr::CYCLE_AVC_VREF
    - HregAddr::CYCLE_AXIS_Z_TORCH_UP_OFFSET
    - HregAddr::CYCLE_Z_STATIC_OFFSET
    - HregAddr::CYCLE_TOUCH_RETRACT_REPOSITION_DISTANCE
    - HregAddr::CYCLE_TOUCH_RETRACT_PROBE_SPEED
    - HregAddr::CYCLE_TOUCH_RETRACT_FINAL_HEIGHT
    - CoilAddr::CYCLE_USE_AVC
    - CoilAddr::CYCLE_USE_TOUCH_RETRACT
    - CoilAddr::WELDER_SIMULATE_MODE
  - static:
    - /* unitless factor| This number divided by 10000 is multiplied by the raw adc voltage reading to get the actual arc voltage */
      ARC_VOLTAGE_CALIBRATION_FACTOR = 60,
    - /* unitless factor| This number divided by 10000 is multiplied by the raw adc current reading to get the actual arc current */
      ARC_CURRENT_CALIBRATION_FACTOR = 61,