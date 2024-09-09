/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [ "./crates/**/*.rs" ],
  theme: {
    extend: {
      transitionProperty: {
        'height': 'height',
      },
    },
  },
  plugins: [require("rippleui")],
}
