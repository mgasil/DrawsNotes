/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
      // include all rust, html and css files in the src directory
      "./app/src/**/*.{rs,html,css}"
  ],
  theme: {
      extend: {},
  },
  plugins: [],
}