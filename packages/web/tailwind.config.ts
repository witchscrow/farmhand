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
				white: '#F5F5F5',
				black: '#2F4F4F',
				primary: {
					'50': '#f4f7f2',
					'100': '#e5ebe0',
					'200': '#cbd7c3',
					'300': '#a5bb9a',
					'400': '#7c9a6f',
					'500': '#5a7c4d',
					'600': '#44613a',
					'700': '#354e2e',
					'800': '#2b3f26',
					'900': '#243420',
					'950': '#131d11'
				},
				secondary: {
					'50': '#f7f4ef',
					'100': '#ebe5d6',
					'200': '#d9cbaf',
					'300': '#c2aa82',
					'400': '#b08f5f',
					'500': '#a17d51',
					'600': '#8a6444',
					'700': '#6f4e39',
					'800': '#5c4033',
					'900': '#523a31',
					'950': '#2f1e19'
				},
				accent: {
					'50': '#fdf9ef',
					'100': '#fbf0da',
					'200': '#f5deb3',
					'300': '#eec683',
					'400': '#e7a550',
					'500': '#e18c2e',
					'600': '#d27324',
					'700': '#af5a1f',
					'800': '#8b4721',
					'900': '#713d1d',
					'950': '#3d1e0d'
				},
				tertiary: {
					'50': '#f2f9fd',
					'100': '#e4f2fa',
					'200': '#c2e5f5',
					'300': '#87ceeb',
					'400': '#50b9e0',
					'500': '#2aa0cd',
					'600': '#1b81ae',
					'700': '#17678d',
					'800': '#175775',
					'900': '#184962',
					'950': '#102f41'
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
