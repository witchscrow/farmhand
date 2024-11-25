<script lang="ts">
	type AlertType = 'error' | 'success' | 'warning' | 'info';
	type AlertSize = 'normal' | 'small';

	interface Props {
		class: string;
		message: string;
		type?: AlertType;
		size?: AlertSize;
	}
	let { class: className, message, type = 'info', size = 'normal' }: Props = $props();
	const styles = {
		error: {
			wrapper: 'bg-red-100 border-2 border-red-300 dark:bg-red-900/75 dark:border-red-600',
			text: 'text-red-800 dark:text-red-100',
			icon: 'text-red-600 dark:text-red-300'
		},
		success: {
			wrapper:
				'bg-primary-100 border-2 border-primary-300 dark:bg-primary-900/75 dark:border-primary-600',
			text: 'text-primary-800 dark:text-primary-100',
			icon: 'text-primary-600 dark:text-primary-300'
		},
		warning: {
			wrapper:
				'bg-secondary-100 border-2 border-secondary-300 dark:bg-secondary-900/75 dark:border-secondary-600',
			text: 'text-secondary-800 dark:text-secondary-100',
			icon: 'text-secondary-600 dark:text-secondary-300'
		},
		info: {
			wrapper:
				'bg-primary-100 border-2 border-primary-300 dark:bg-primary-900/75 dark:border-primary-600',
			text: 'text-primary-800 dark:text-primary-100',
			icon: 'text-primary-600 dark:text-primary-300'
		}
	};
</script>

<div
	class="flex items-center justify-center rounded-md p-2 text-center {styles[type]
		.wrapper} {className}"
>
	<div class="flex-shrink-0">
		{#if type === 'error'}
			<svg
				class="{size === 'small' ? 'h-4 w-4' : 'h-5 w-5'} {styles[type].icon}"
				viewBox="0 0 20 20"
				fill="currentColor"
				aria-hidden="true"
			>
				<path
					fill-rule="evenodd"
					d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.28 7.22a.75.75 0 00-1.06 1.06L8.94 10l-1.72 1.72a.75.75 0 101.06 1.06L10 11.06l1.72 1.72a.75.75 0 101.06-1.06L11.06 10l1.72-1.72a.75.75 0 00-1.06-1.06L10 8.94 8.28 7.22z"
					clip-rule="evenodd"
				/>
			</svg>
		{:else if type === 'success'}
			<svg
				class="{size === 'small' ? 'h-4 w-4' : 'h-5 w-5'} {styles[type].icon}"
				viewBox="0 0 20 20"
				fill="currentColor"
				aria-hidden="true"
			>
				<path
					fill-rule="evenodd"
					d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
					clip-rule="evenodd"
				/>
			</svg>
		{:else if type === 'warning'}
			<svg
				class="{size === 'small' ? 'h-4 w-4' : 'h-5 w-5'} {styles[type].icon}"
				viewBox="0 0 20 20"
				fill="currentColor"
				aria-hidden="true"
			>
				<path
					fill-rule="evenodd"
					d="M8.485 2.495c.673-1.167 2.357-1.167 3.03 0l6.28 10.875c.673 1.167-.17 2.625-1.516 2.625H3.72c-1.347 0-2.189-1.458-1.515-2.625L8.485 2.495zM10 5a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 0110 5zm0 9a1 1 0 100-2 1 1 0 000 2z"
					clip-rule="evenodd"
				/>
			</svg>
		{:else if type === 'info'}
			<svg
				class="{size === 'small' ? 'h-4 w-4' : 'h-5 w-5'} {styles[type].icon}"
				viewBox="0 0 20 20"
				fill="currentColor"
				aria-hidden="true"
			>
				<path
					fill-rule="evenodd"
					d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a.75.75 0 000 1.5h.253a.25.25 0 01.244.304l-.459 2.066A1.75 1.75 0 0010.747 15H11a.75.75 0 000-1.5h-.253a.25.25 0 01-.244-.304l.459-2.066A1.75 1.75 0 009.253 9H9z"
					clip-rule="evenodd"
				/>
			</svg>
		{/if}
	</div>
	<div class="ml-3">
		<p class="text-xs font-medium {styles[type].text}">
			{message}
		</p>
	</div>
</div>
