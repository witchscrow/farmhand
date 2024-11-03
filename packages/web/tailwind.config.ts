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
				black: '#1f1f1f',
				white: '#fefcd9',
				primary: {
					'50': '#c9ffd8',
					'100': '#a3ffc2',
					'200': '#7affaa',
					'300': '#42ff89',
					'400': '#00f25d',
					'500': '#00ca49',
					'600': '#009e3c',
					'700': '#00883a',
					'800': '#02652e',
					'900': '#003917',
					'950': '#00240f'
				},
				secondary: {
					'50': '#fdfee8',
					'100': '#fcffbe',
					'200': '#fdff88',
					'300': '#fffa44',
					'400': '#feed11',
					'500': '#eed404',
					'600': '#cda601',
					'700': '#a47704',
					'800': '#875d0c',
					'900': '#734c10',
					'950': '#432805'
				}
			},
			gridTemplateRows: {
				main: 'min-content 1fr min-content'
			}
		},
		fontFamily: {
			sans: ['Outfit', 'sans-serif'],
			serif: ['DM Serif Display', 'serif'],
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
			xs: '0.85rem',
			sm: '1rem',
			base: ['1.1rem', { lineHeight: '1.25rem' }],
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
