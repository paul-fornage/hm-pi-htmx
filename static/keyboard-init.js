(() => {
  "use strict";

  const Keyboard = window.SimpleKeyboard && window.SimpleKeyboard.default;
  if (!Keyboard) {
    console.warn("simple-keyboard not found");
    return;
  }

  let keyboardLayer = document.getElementById("global-keyboard-layer");
  let keyboardRoot = document.getElementById("global-keyboard");
  if (!keyboardLayer || !keyboardRoot) {
    return;
  }

  const layouts = {
    float: {
      default: ["7 8 9 {bksp}", "4 5 6 {clear}", "1 2 3 {close}", "- 0 . {enter}"],
    },
    text: {
      default: [
        "` 1 2 3 4 5 6 7 8 9 0 - = {bksp}",
        "{tab} q w e r t y u i o p [ ] \\",
        "{lock} a s d f g h j k l ; ' {enter}",
        "{shift} z x c v b n m , . / {shift}",
        "{space}"
      ],
      shift: [
        "~ ! @ # $ % ^ & * ( ) _ + {bksp}",
        "{tab} Q W E R T Y U I O P { } |",
        "{lock} A S D F G H J K L : \" {enter}",
        "{shift} Z X C V B N M < > ? {shift}",
        "{space}"
      ]
    }
  };

  const display = {
    "{bksp}": "⇤ delete",
    "{enter}": "enter ↵",
    "{shift}": "shift",
    "{close}": "⨯ close",
    "{space}": "space",
    "{clear}": "🗑︎ clear",
  };

  let activeInput = null;
  let activeType = null;
  let activeInputHandler = null;

  const keyboard = new Keyboard(keyboardRoot, {
    layout: layouts.float,
    layoutName: "default",
    mergeDisplay: true,
    display,
    buttonTheme: [
      { class: "sk-key-danger", buttons: "{bksp} {clear}" },
      { class: "sk-key-neutral", buttons: "{shift} {lock} {tab} {space} {close}" },
      { class: "sk-key-success", buttons: "{enter}" }
    ],
    preventMouseDownDefault: true,
    onChange: handleKeyboardChange,
    onKeyPress: handleKeyPress
  });

  function keyboardTypeForInput(input) {
    if (!(input instanceof HTMLInputElement || input instanceof HTMLTextAreaElement)) {
      return null;
    }

    const dataType = input.dataset && input.dataset.keyboard;
    if (dataType === "float") {
      return "float";
    }

    if (dataType === "text") {
      return "text";
    }

    return null;
  }

  function handleKeyboardChange(input) {
    if (!activeInput || !activeInput.isConnected) {
      hideKeyboard();
      return;
    }

    if (activeInput.value !== input) {
      activeInput.value = input;
      activeInput.dispatchEvent(new Event("input", { bubbles: true }));
    }
  }

  function handleKeyPress(button) {
    if (button === "{close}") {
      if (activeInput) {
        activeInput.blur();
      }
      return;
    }

    if (button === "{clear}") {
      keyboard.clearInput();
      if (activeInput) {
        activeInput.value = "";
        // Dispatch input event so frameworks/listeners detect the change
        activeInput.dispatchEvent(new Event("input", { bubbles: true }));
      }
      return;
    }

    if (activeType === "text" && (button === "{shift}" || button === "{lock}")) {
      toggleShift();
    }
  }

  function toggleShift() {
    const currentLayout = keyboard.options.layoutName;
    const nextLayout = currentLayout === "default" ? "shift" : "default";
    keyboard.setOptions({ layoutName: nextLayout });
  }

  function setActiveInput(input, type) {

    keyboardLayer = document.getElementById("global-keyboard-layer");
    keyboardRoot = document.getElementById("global-keyboard");
    if (!keyboardLayer || !keyboardRoot) {
      console.warn("simple-keyboard not found in DOM");
      return;
    }

    keyboardRoot.classList.toggle("keyboard-float", type === "float");
    keyboardRoot.classList.toggle("keyboard-text", type === "text");

    if (activeInput && activeInputHandler) {
      activeInput.removeEventListener("input", activeInputHandler);
    }

    activeInput = input;
    activeInputHandler = () => {
      if (!activeInput || !activeInput.isConnected) {
        hideKeyboard();
        return;
      }
      keyboard.setInput(activeInput.value || "");
    };
    activeInput.addEventListener("input", activeInputHandler);

    if (activeType !== type) {
      activeType = type;
      keyboard.setOptions({
        layout: layouts[type] || layouts.float,
        layoutName: "default"
      });
    }

    keyboard.setInput(activeInput.value || "");
  }

  function showKeyboard() {
    keyboardLayer.hidden = false;
  }

  function hideKeyboard() {
    if (activeInput && activeInputHandler) {
      activeInput.removeEventListener("input", activeInputHandler);
    }
    activeInput = null;
    activeInputHandler = null;
    activeType = null;
    keyboardLayer.hidden = true;
  }

  document.addEventListener("focusin", (event) => {
    const target = event.target;
    const type = keyboardTypeForInput(target);
    if (!type) {
      return;
    }

    setActiveInput(target, type);
    showKeyboard();
  });

  document.addEventListener("focusout", (event) => {
    if (event.target !== activeInput) {
      return;
    }

    hideKeyboard();
  });

})();
