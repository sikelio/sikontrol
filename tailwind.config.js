/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './index.html',
    './public/splashscreen.html',
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
      },
      boxShadow: {
        sidebar: '0 10px 20px -3px rgba(255, 255, 255, 0.2), 0 4px 6px -2px rgba(255, 255, 255, 0.1)',
      },
      backgroundImage: {
        logo: "url('/logo.png')"
      }
    },
  },
  plugins: [
    require('daisyui')
  ],
}
