(function() {
    // --- 1. Configurations ---
    const CONFIGS = {
        float: {
            keysArrayOfObjects: [
                { "0": "7", "1": "8", "2": "9" },
                { "0": "4", "1": "5", "2": "6" },
                { "0": "1", "1": "2", "2": "3" },
                { "0": "-", "1": "0", "2": "." },
                { "0": "Backspace", "1": "Clear", "2": "Enter" }
            ],
            keysJsonUrl: null,
            language: 'en',
            theme: 'light',
            capsLockActive: false,
            cssAnimations: true,
            allowRealKeyboard: true,
        },
        text: {
            keysJsonUrl: null,
            language: 'en',
            theme: 'light',
            capsLockActive: true,
            cssAnimations: true,
            keysAllowSpacebar: true,
            allowRealKeyboard: true,
        }
    };

    // --- 2. JIT Initialization (The "On Focus" logic) ---
    // We use 'pointerdown' (click/touch) because it fires BEFORE 'focus'.
    // This gives us a split second to attach the keyboard before the browser focuses the input.
    document.addEventListener('pointerdown', function(e) {
        const target = e.target;

        // 1. Check if it's one of our inputs
        if (!target.matches || (!target.matches('.keyboard-float') && !target.matches('.keyboard-text'))) {
            return;
        }

        // 2. Check if already initialized (prevent double-binding)
        if (target.dataset.kbReady) return;

        // 3. Initialize KioskBoard for this specific class of inputs
        // Note: calling run() repeatedly is safe, it just re-scans for that selector.
        // Since we are inside a user interaction, the performance cost is negligible.
        if (target.matches('.keyboard-float')) {
            KioskBoard.run('.keyboard-float', CONFIGS.float);
        } else {
            KioskBoard.run('.keyboard-text', CONFIGS.text);
        }

        // 4. Mark this specific element as ready
        target.dataset.kbReady = "true";
    });


    // --- 3. Validation Logic (Event Delegation) ---
    // We listen to the DOCUMENT, so this works for existing AND new HTMX elements automatically.

    document.addEventListener('input', function(e) {
        if (e.target.matches && e.target.matches('.keyboard-float')) {
            validateFloat(e.target);
        }
    });

    function validateFloat(input) {
        const valStr = input.value;

        // Clear error if empty (unless required, which HTML handles)
        if (valStr === '') {
            input.setCustomValidity('');
            return;
        }

        const val = parseFloat(valStr);

        // Grab attributes
        const min = input.getAttribute('min');
        const max = input.getAttribute('max');
        const step = input.getAttribute('step');

        let error = '';

        if (isNaN(val)) {
            error = 'Invalid number';
        } else if (min !== null && val < parseFloat(min)) {
            error = `Min: ${min}`;
        } else if (max !== null && val > parseFloat(max)) {
            error = `Max: ${max}`;
        } else if (step !== null) {
            const stepNum = parseFloat(step);
            // Check precision (avoiding float math weirdness)
            const remainder = (val / stepNum) % 1;
            if (remainder > 0.0001 && remainder < 0.9999) {
                error = `Step: ${step}`;
            }
        }

        input.setCustomValidity(error);
    }

})();