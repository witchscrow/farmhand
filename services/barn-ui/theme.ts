import type { CustomThemeConfig } from '@skeletonlabs/tw-plugin';

export const farmhandTheme: CustomThemeConfig = {
	name: 'farmhand',
	properties: {
		// =~= Theme Properties =~=
		'--theme-font-family-base': `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
		'--theme-font-family-heading': `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
		'--theme-font-color-base': '0 0 0',
		'--theme-font-color-dark': '255 255 255',
		'--theme-rounded-base': '4px',
		'--theme-rounded-container': '4px',
		'--theme-border-base': '4px',
		// =~= Theme On-X Colors =~=
		'--on-primary': '0 0 0',
		'--on-secondary': '0 0 0',
		'--on-tertiary': '0 0 0',
		'--on-success': '0 0 0',
		'--on-warning': '0 0 0',
		'--on-error': '0 0 0',
		'--on-surface': '0 0 0',
		// =~= Theme Colors  =~=
		// primary | #2cb54e
		'--color-primary-50': '223 244 228', // #dff4e4
		'--color-primary-100': '213 240 220', // #d5f0dc
		'--color-primary-200': '202 237 211', // #caedd3
		'--color-primary-300': '171 225 184', // #abe1b8
		'--color-primary-400': '107 203 131', // #6bcb83
		'--color-primary-500': '44 181 78', // #2cb54e
		'--color-primary-600': '40 163 70', // #28a346
		'--color-primary-700': '33 136 59', // #21883b
		'--color-primary-800': '26 109 47', // #1a6d2f
		'--color-primary-900': '22 89 38', // #165926
		// secondary | #ae9847
		'--color-secondary-50': '243 240 227', // #f3f0e3
		'--color-secondary-100': '239 234 218', // #efeada
		'--color-secondary-200': '235 229 209', // #ebe5d1
		'--color-secondary-300': '223 214 181', // #dfd6b5
		'--color-secondary-400': '198 183 126', // #c6b77e
		'--color-secondary-500': '174 152 71', // #ae9847
		'--color-secondary-600': '157 137 64', // #9d8940
		'--color-secondary-700': '131 114 53', // #837235
		'--color-secondary-800': '104 91 43', // #685b2b
		'--color-secondary-900': '85 74 35', // #554a23
		// tertiary | #47b8a1
		'--color-tertiary-50': '227 244 241', // #e3f4f1
		'--color-tertiary-100': '218 241 236', // #daf1ec
		'--color-tertiary-200': '209 237 232', // #d1ede8
		'--color-tertiary-300': '181 227 217', // #b5e3d9
		'--color-tertiary-400': '126 205 189', // #7ecdbd
		'--color-tertiary-500': '71 184 161', // #47b8a1
		'--color-tertiary-600': '64 166 145', // #40a691
		'--color-tertiary-700': '53 138 121', // #358a79
		'--color-tertiary-800': '43 110 97', // #2b6e61
		'--color-tertiary-900': '35 90 79', // #235a4f
		// success | #2495ff
		'--color-success-50': '222 239 255', // #deefff
		'--color-success-100': '211 234 255', // #d3eaff
		'--color-success-200': '200 229 255', // #c8e5ff
		'--color-success-300': '167 213 255', // #a7d5ff
		'--color-success-400': '102 181 255', // #66b5ff
		'--color-success-500': '36 149 255', // #2495ff
		'--color-success-600': '32 134 230', // #2086e6
		'--color-success-700': '27 112 191', // #1b70bf
		'--color-success-800': '22 89 153', // #165999
		'--color-success-900': '18 73 125', // #12497d
		// warning | #ece74b
		'--color-warning-50': '252 251 228', // #fcfbe4
		'--color-warning-100': '251 250 219', // #fbfadb
		'--color-warning-200': '250 249 210', // #faf9d2
		'--color-warning-300': '247 245 183', // #f7f5b7
		'--color-warning-400': '242 238 129', // #f2ee81
		'--color-warning-500': '236 231 75', // #ece74b
		'--color-warning-600': '212 208 68', // #d4d044
		'--color-warning-700': '177 173 56', // #b1ad38
		'--color-warning-800': '142 139 45', // #8e8b2d
		'--color-warning-900': '116 113 37', // #747125
		// error | #dd3636
		'--color-error-50': '250 225 225', // #fae1e1
		'--color-error-100': '248 215 215', // #f8d7d7
		'--color-error-200': '247 205 205', // #f7cdcd
		'--color-error-300': '241 175 175', // #f1afaf
		'--color-error-400': '231 114 114', // #e77272
		'--color-error-500': '221 54 54', // #dd3636
		'--color-error-600': '199 49 49', // #c73131
		'--color-error-700': '166 41 41', // #a62929
		'--color-error-800': '133 32 32', // #852020
		'--color-error-900': '108 26 26', // #6c1a1a
		// surface | #1f1f1f
		'--color-surface-50': '221 221 221', // #dddddd
		'--color-surface-100': '210 210 210', // #d2d2d2
		'--color-surface-200': '199 199 199', // #c7c7c7
		'--color-surface-300': '165 165 165', // #a5a5a5
		'--color-surface-400': '98 98 98', // #626262
		'--color-surface-500': '31 31 31', // #1f1f1f
		'--color-surface-600': '28 28 28', // #1c1c1c
		'--color-surface-700': '23 23 23', // #171717
		'--color-surface-800': '19 19 19', // #131313
		'--color-surface-900': '15 15 15' // #0f0f0f
	}
};
