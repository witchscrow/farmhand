<script lang="ts">
	export let title: string;
	export let description: string;
	export let icon: string;
</script>

<div
	class="relative rounded-lg border border-primary-200/20 bg-secondary-400 p-6 shadow-lg backdrop-blur-sm transition-all hover:bg-primary-900 dark:border-primary-900/40 dark:bg-primary-800 dark:hover:bg-black"
>
	<h3 class="mb-2 text-base font-medium text-secondary-900 dark:text-white">
		<span class="text-xs">{icon}</span>
		{title}
	</h3>
	<p class="text-sm text-secondary-800 dark:text-primary-100/80">
		{description}
	</p>
</div>
