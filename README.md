

`tailwindcss -i styles/input.css -o static/assets/main.css`

See SSE HTMX for next steps

TODO: 

- [ ] load to welder with diff
- [ ] The fs modals block out the rest of the app in black. They should be semi transparent like every other modal in the project.
- [ ] After pressing the final load button, it should disable and show loading while the request is in flight.
- [ ] The profile name and description needs to update when changed. Use HTMX SSE (already installed to trigger these). The server will change these values when: loading a new profile, "save as" with a new name. Deleting current profile. (maybe more?)
- [ ] When overwriting exisitng profiles, it should not use the browser pop up. It should use the same idea as the delete button in the list of 'load' profiles
- [ ] unified feedback across whole app with OOBS and OOBS template?
- [ ] In edit analog modal, the precision should be obvious. (maybe current value rendered in full precision)