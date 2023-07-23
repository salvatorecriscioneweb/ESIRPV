/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [],
  purge: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        bg: '#EEEEEE',
        primary: '#068FFF',
      },
      fontSize: {
        h1: '4.209rem',
        h2: '3.157rem',
        h3: '2.369rem',
        h4: '1.777rem',
        base: '1.333rem',
        small: '1rem',
      },
    },
  },
  plugins: [],
}
