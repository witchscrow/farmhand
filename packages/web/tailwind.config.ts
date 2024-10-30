import aspectRatio from '@tailwindcss/aspect-ratio';
import containerQueries from '@tailwindcss/container-queries';
import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';
import type { Config } from 'tailwindcss';

export default {
	darkMode: 'selector',
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				primary: {
					'50': '#f3f6ef',
					'100': '#e5ebdc',
					'200': '#ccd9bd',
					'300': '#adc195',
					'400': '#8fa972',
					'500': '#728d55',
					'600': '#586f41',
					'700': '#4d613b',
					'800': '#3a462e',
					'900': '#323d2a',
					'950': '#192013'
				},
				secondary: {
					'50': '#f5fde8',
					'100': '#e8f9ce',
					'200': '#d0f39d',
					'300': '#b5ea6c',
					'400': '#98dc3f',
					'500': '#79c121',
					'600': '#5b9a16',
					'700': '#467615',
					'800': '#3a5d17',
					'900': '#334f18',
					'950': '#182c07'
				}
			}
		},
		fontFamily: {
			sans: ['Outfit', 'sans-serif'],
			mono: [
				'ui-monospace',
				'SFMono-Regular',
				'Menlo',
				'Monaco',
				'Consolas',
				'Liberation Mono',
				'Courier New',
				'monospace'
			]
		},
		fontSize: {
			xs: '0.9rem',
			sm: '1rem',
			base: ['1.5rem', { lineHeight: '1.8rem' }],
			lg: ['1.75rem', { lineHeight: '2rem' }],
			xl: ['2rem', { lineHeight: '2.2rem' }],
			'2xl': ['3.2rem', { lineHeight: '3.4rem' }],
			'3xl': ['3.8rem', { lineHeight: '4rem' }],
			'4xl': ['4.5rem', { lineHeight: '4.7rem' }],
			'5xl': ['5.5rem', { lineHeight: '5.7rem' }],
			'6xl': ['6.5rem', { lineHeight: '6.7rem' }],
			'7xl': ['7.5rem', { lineHeight: '7.7rem' }],
			'8xl': ['8.5rem', { lineHeight: '8.7rem' }],
			'9xl': ['9.5rem', { lineHeight: '9.7rem' }]
		}
	},
	plugins: [typography, forms, containerQueries, aspectRatio]
} satisfies Config;
