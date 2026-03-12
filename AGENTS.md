# Rust/hm-pi-htmx

- Rust code lives in `src`
- HTML templates live in `templates`
- When using format! and you can inline variables into {}, always do that.
- 
- Always collapse if statements per https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_if
- Always inline format! args when possible per https://rust-lang.github.io/rust-clippy/master/index.html#uninlined_format_args
- Use method references over closures when possible per https://rust-lang.github.io/rust-clippy/master/index.html#redundant_closure_for_method_calls
- When possible, make `match` statements exhaustive and avoid wildcard arms.
- Do not create small helper methods that are referenced only once.
- Avoid large modules:
    - Prefer adding new modules instead of growing existing ones.
    - Target Rust modules under 500 LoC, excluding tests.
    - If a file exceeds roughly 800 LoC, add new functionality in a new module instead of extending
      the existing file unless there is a strong documented reason not to.

## Project context

This code interfaces with a Miller welder and a clearcore (an MCU) via a modbus protocol. This project only runs on one machine, on a raspberry pi with a touchscreen. The system is airgapped, and security is not a concern. Bad actors will need physical access to an industrial military complex to do anything, and at that point we will not be the target.

## Tests

This project is not easy to test, and I don't think they should be forced in. However, when possible, write debug assertions and tests for the few pure functional facets of the code. This excludes the modbus protocol implementation, which is handled by humans only.

### HTML Rules

- When writing HTML, try to use the least amount of js possible.
- Always check if HTMX provides a way to do what you want natively.
- When doing styling, focus on the layout. Plan how it should look, and then acheive the layout using the least amount of CSS possible.
- Always prefer tailwind over CSS. If you need to use CSS, leave a comment explaining why.