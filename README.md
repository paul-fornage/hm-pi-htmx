

`tailwindcss -i styles/input.css -o static/assets/main.css`

See SSE HTMX for next steps


```clankerprompt
Can you fix the sign in endpoint and modal? `src/views/auth.rs` (not `src/auth.rs`)

I noticed that after signing in, the header does not update to show the newly accessible endpoints, or the username and auth level. Luckily, there is already an htmx trigger emitted for auth change, so it should be pretty simple. Also if the user is already on a page at time of sign out that isn't accessible to the signed out operator, it should boot them back to manual control. The trigger should trigger a specific element, not global if possible. Also if it's not too hard, use the HxTrigger struct! 
`src/hx_trigger.rs`
Example usage in `src/views/welder_profile/file_system_templates.rs`

Does this make sense?

Tell me how you plan on doing this, NO GETTING THE URL
```



TODO: 
- [ ] Sign out changes tabs
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