<script lang="ts">
	export let type: 'error' | 'success' | 'warning' | 'info' = 'info';
	export let message: string;

	const styles = {
		error: {
			wrapper: 'bg-red-50 dark:bg-red-900/50',
			text: 'text-red-700 dark:text-red-200',
			icon: 'text-red-400 dark:text-red-300'
		},
		success: {
			wrapper: 'bg-green-50 dark:bg-green-900/50',
			text: 'text-green-700 dark:text-green-200',
			icon: 'text-green-400 dark:text-green-300'
		},
		warning: {
			wrapper: 'bg-yellow-50 dark:bg-yellow-900/50',
			text: 'text-yellow-700 dark:text-yellow-200',
			icon: 'text-yellow-400 dark:text-yellow-300'
		},
		info: {
			wrapper: 'bg-blue-50 dark:bg-blue-900/50',
			text: 'text-blue-700 dark:text-blue-200',
			icon: 'text-blue-400 dark:text-blue-300'
		}
	};
</script>

<div class="mb-4 flex items-center justify-start rounded-md p-4 {styles[type].wrapper}">
	<div class="flex-shrink-0">
		{#if type === 'error'}
			<svg
				class="h-5 w-5 {styles[type].icon}"
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
		{/if}
	</div>
	<div class="ml-3">
		<p class="text-sm font-medium {styles[type].text}">{message}</p>
	</div>
</div>
