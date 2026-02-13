

`tailwindcss -i styles/input.css -o static/assets/main.css`

See SSE HTMX for next steps

TODO: 
- [ ] clearcore UDP messages should use ascii control chars to not log line numbers and file name to front end alerts
- [ ] update speeds as part of the motion profile
- [ ] preemptive frontend feedback for things like mandrel must be closed or fingers not down
- [ ] jog buttons on touch screen don't work
- [ ] estop in front end (display and button)
- [ ] text inputs with numeric min and max and step
- [ ] `autocomplete="off"` for inputs - maybe combine with above for a custom input component?
- [ ] When overwriting existing profiles, it should not use the browser pop up. It should use the same idea as the delete button in the list of 'load' profiles
  - logging:
    - [ ] Implement a memory buffer to store the last N messages
    - [ ] Create a frontend page that displays the initial buffer history
    - [ ] Integrate the page with Server-Sent Events for live appending
    - [ ] Configure SSE to trigger toast notifications for error logs