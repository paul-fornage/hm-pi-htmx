

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
- [ ] add a blank space for the keyboard to the bottom of the aadmin adjustment editor view.

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


I'm adding a new feature to my firmware, when a user is on the 'run cycle' page, at the bottom should be a form that lets them apply 'adjustments' within ranges laid out by admins in the 'operator adjustment' view. Idea being, an admin can decide what parameters can be tweaked by operators, and by how much, and then when an operator goes to run a cycle, they can choose a preset and apply their adjustments. Look at the project for some context and then start with step one I included below.

## Step 1
In the run-cycles page, add a view at the bottom that displays the preset value of all the analog registers in a table. Please re-use Rust from where they are edited, e.g. the code that defines the conversions and whatnot, not the html. For now, skip special case registers like 'post flow time', that is an enum and a scalar. When there is no preset selected, do not display anything. Create an event that is triggered when loading presets is successful that triggers this endpoint. On the first load, check if selected presets are already loaded, but also make sure I didn't already write this functionality. If I did, and the page already pre-loads something showing if the preset is already loaded, just add the event to the response. I already have an HxTrigger struct that I can use to trigger events. Please use that.

## Step 2
Add a function that takes the adjustment ranges set by the admin, and calculates the real semantic min and max. Double check that these values do not exceed the register max value. e.g. if a register has a max value of 1000.0, currently 900.0, and the adjustment range is +/- 50%, the computed min and max should be `450.0..1000.0`

## Step 3
Change the list from step 1 to be a form. The form should use structured json with htmx-ext-json-form. It should use the actual preset values on load, but let the user change them within the ranges. Look at other input fields in the project, they need to support a keyboard, and need to actually be type="text". Also means that at the bottom of the list, there needs to be like 30vh spacing so the keyboard doesn't cover the last items.

## Step 4
Add a checkbox or toggle next to the 'run' buttons that optionally uses the 'modified' preset values. When checked you can guess what this does. There need to be error handlers to verify readback of values written. It's possible that the welder has a dynamic limit on these fields, and might not work with them. Because the run command now takes longer, use styling for both run buttons while requests are in flight, and obviously disable them.

## Step 5
Add the info modal to the registers. I've made info modals for these before, so try to re-use those, but I think the API for the old ones won't work with the fact that until submitted, the value only exists in the client. If that's the case, just show the preset value.

Notice none of these tasks actually work on the modbus values. Always work on the presets as the source of truth.

General rules:

No silent errors. Log message or user feedback. If you are ever using a default value, you need to have a damn good reason.

If you leave something unimplemented, PLEASE add a `// TODO:` comment so my IDE picks it up, or even better a `todo!()` macro.

Do not duplicate work. I'm not saying don't repeat yourself, I don't really care about that, I'm saying DO NOT implement things again that already exist elsewhere in the codebase. If it's there and not 'exported', Please leave a TODO showing where the code could be merged, but don't modify other functionality for now.

Ask if you have a question, or maybe make a list, but do not ask trivial questions about color or string formatting. If you are making decisions that will shape what I have to do next, like architectural decisions, then ask.