<script lang="ts">
	export let title: string;
	export let description: string;
	export let icon: string;
</script>

<div
	class="rounded-sm border-4 border-secondary-600 bg-secondary-100/90 p-6 text-left shadow-sm shadow-primary-800 dark:border-primary-600 dark:bg-primary-800/90"
>
	<h3 class="mb-2 text-secondary-800 dark:text-white">
		<span class="text-xs">{icon}</span>
		{title}
	</h3>
	<p class="text-sm text-gray-600 dark:text-gray-300">
		{description}
	</p>
</div>
