/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './index.html',
    './src/**/*.{js,ts}',
  ],
  theme: {
    extend: {
      fontFamily: {
        'montserrat': ['Montserrat', 'sans-serif'],
      },
      colors: {
        'app-light-gray': '#363636',
        'app-dark-gray': '#222222'
      }
    },
  },
  plugins: [
    require('daisyui')
  ],
}
