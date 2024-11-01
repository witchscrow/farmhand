<script lang="ts">
	export let title: string;
	export let description: string;
	export let icon: string;
</script>

<div class="rounded-lg border border-gray-200 p-6 dark:border-gray-700">
	<div class="mb-4 text-3xl">{icon}</div>
	<h3 class="mb-2 text-lg font-semibold text-primary-700 dark:text-primary-200">
		{title}
	</h3>
	<p class="text-sm text-gray-600 dark:text-gray-300">
		{description}
	</p>
</div>
