import aspectRatio from '@tailwindcss/aspect-ratio';
import containerQueries from '@tailwindcss/container-queries';
import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';
import type { Config } from 'tailwindcss';
import { skeleton } from '@skeletonlabs/tw-plugin';
import { join } from 'path';
import { farmhandTheme } from './theme';

export default {
	darkMode: 'selector',
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		join(require.resolve('@skeletonlabs/skeleton'), '../**/*.{html,js,svelte,ts}')
	],
	theme: {
		extend: {
			colors: {
				'brand-twitch': '#9146FF'
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
	plugins: [
		typography,
		forms,
		containerQueries,
		aspectRatio,
		skeleton({
			themes: {
				custom: [farmhandTheme]
			}
		})
	]
} satisfies Config;
