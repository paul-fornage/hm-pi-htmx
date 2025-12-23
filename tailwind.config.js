
/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./templates/**/*.html",
        "./src/**/*.rs"
    ],
    theme: {
        extend: {
            fontFamily: {
                'mono': ['JetBrains Mono', 'monospace'],
            },
        },
    },
    plugins: [],
}