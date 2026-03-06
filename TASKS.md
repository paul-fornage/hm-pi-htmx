# USB transfer view: tasks + spec

Context
- Current view: `src/views/usb_transfer.rs` and templates under `templates/views/usb-transfer.html`.
- USB discovery helpers: `src/paths/usb_drives.rs` (`usb_mountpoints()`).
- Local data roots/subdirs: `src/paths/subdirs.rs`, `src/paths/mod.rs` (including `DEFAULT_ROOT_FOLDER` and `Subdir::path_in_usb_root(...)`).
- Local subdir cache: `pub static LOCAL_SUBDIR_PATHS: LazyLock<SubdirPaths>` in `src/main.rs` (use this to avoid recomputing local paths).
- HTMX trigger helper: `src/hx_trigger.rs`.

Non-negotiables
- NEVER produce silent errors. If we fall back to defaults, the caller must be able to see the value is untrustworthy and we must log the reason.
- Log errors and unusual conditions, but avoid spam (no per-file render logs).

## Clarified decisions (from user)
- File filter: define `const ALLOWED_EXTENSION: &str = "json"` and list/copy only files with that extension.
- Display: show file names without extension to the user.
- Directory depth: no recursion; files only at subdir root.
- Subdir scope: include ALL `Subdir::VARIANTS`.
- Confirm modal: use a dedicated confirm overwrite modal (not `#global-modal-layer`).
- USB structure: build USB paths with `Subdir::path_in_usb_root(usb_mountpoint)` which yields `{mount}/{DEFAULT_ROOT_FOLDER}/{subdir.path()}`.
- USB folder initialization: verify mountpoint exists; if missing, error as “device removed before operation.” If mount exists, create `{DEFAULT_ROOT_FOLDER}` then the subdir folder step-by-step (no `create_dir_all`), so we never create a fake mount if the device is removed between checks.
- Local paths: read from `LOCAL_SUBDIR_PATHS` in `src/main.rs` instead of recomputing per request.

## Open question
- If the copy form posts a `usb_mountpoint` that no longer exists (device removed), should we always return a user-facing error and skip any further work?

## 1. Scoping + API design
- Define new module layout for USB transfer view:
  - Route handler module for HTTP/HTMX (new file under `src/views/usb_transfer/`).
  - File transfer module(s) for file I/O + copy logic.
  - Keep `src/views/usb_transfer.rs` as a small module entrypoint (re-export + routes).
- Define form struct for copy endpoint:
  - fields: `file_name` (base name without extension), `subdir`, `direction` enum (usb->local | local->usb), `usb_mountpoint`, `force` (default false).
  - Ensure `force=true` cannot return Confirm.
- Define response enum:
  - `Success { reload_target: ReloadTarget(usb|local|both), message: String }`
  - `Error { message: String }`
  - `Confirm { form: FormStruct }`
- Implement `IntoResponse` for the response enum:
  - `Success`: set `HxTrigger` header to reload the correct list(s), render success toast template.
  - `Error`: set `HxTrigger` header for both lists, render error toast template.
  - `Confirm`: no `HxTrigger`, render confirm overwrite modal template with hidden fields pre-filled and `force=true`.

## 2. USB transfer view UI + templates
- Update `templates/views/usb-transfer.html` and create component templates as needed:
  - Two lists side-by-side: local files by `Subdir` and USB files by `Subdir`.
  - Each file row has a copy button targeting the opposite side.
  - Buttons show spinner/disabled state during HTMX request.
  - Take pride in the UX. Write it like you care about what your boss thinks of it.
  - Try not to spam Tailwind classes. Start with the UI layout in your head and then use the least CSS possible to get it working.
  - Avoid using absolute positioning for CSS layout.
  - Use modern HTMX. Chances are, whatever you're doing already has a nice dedicated attribute.
  - Use the least JS possible.
- Add a manual refresh control; larger/more prominent when no drives are found.
- Display drive info nicely (name, mountpoint, size; uses `UsbDrive::size_display()`).
- If multiple USB drives are mounted, pre-populate the toast with a message explaining selection behavior (we’ll still default to the first per `usb_mountpoints()` sorting).

## 3. Data loading + routing
- Add new routes under the USB transfer view:
  - Page route (existing) for the full view.
  - List partials:
    - Local list partial endpoint.
    - USB list partial endpoint.
  - Copy endpoint (HTMX POST or PUT): accepts the form struct.
- Page load should pick the first USB drive using the same logic as `usb_mountpoints()`.
  - If >1 drive, include toast message warning about the selection.
  - If 0 drives, show no-drive state and large refresh control.

## 4. File transfer logic
- Create file transfer helpers for:
  - Listing local files by `Subdir` (filter by `ALLOWED_EXTENSION` only).
  - Listing USB files by `Subdir` for the selected mountpoint (filter by `ALLOWED_EXTENSION` only).
  - Copying files with collision handling:
    - If destination file exists and `force=false`, return `Confirm` response with pre-filled form and `force=true`.
    - If `force=true`, overwrite directly and never return `Confirm`.
  - Construct file paths by appending `.{ALLOWED_EXTENSION}` to `file_name`.
  - Ensure mountpoint exists before any USB operation; error if missing.
  - Create USB folders step-by-step:
    - `{mount}/{DEFAULT_ROOT_FOLDER}`
    - `{mount}/{DEFAULT_ROOT_FOLDER}/{subdir}`.
  - Validate inputs (no traversal).
- Log only on:
  - Operation start/end per request.
  - Errors (read/list/copy/canonicalize), with enough context to debug.

## 5. HTMX behavior
- Copy button sends HTMX request to copy endpoint, targeting toast div.
- On `Success`:
  - Return toast content.
  - Trigger reload of destination list via `HxTrigger` (and possibly source list if required by `ReloadTarget`).
- On `Error`:
  - Return error toast content.
  - Trigger reload of both lists (per requirement).
- On `Confirm`:
  - Return modal content (OOB swap if needed).
  - No triggers.

## 6. Wiring + exports
- Update `src/views/mod.rs` to use the new module layout and routes.
- Ensure new modules are in `src/views/usb_transfer/` and exported as needed.

## 7. Tests and manual checks
- Add or update any unit tests if the project has existing coverage for file operations or view routes.
- Manual checks (documented in comments or README if needed):
  - Copy local->USB and USB->local.
  - Confirm overwrite flow appears only when `force=false` and file exists.
  - Toast is updated on success/error; destination list reloads.
  - Multiple USB drive warning appears.
  - No-drive state is clear and refresh control works.
