/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./{frontend,server}/**/*.{rs,html}"],
  theme: {
    extend: {
      fontFamily: {
        'serif': ['"Merriweather"', 'serif'],
        'sans': ['"Montserrat"', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
