<script lang="ts">
	export let label: string;
	export let name: string;
	export let type: string = 'text';
	export let value: string = '';
	export let placeholder: string = '';
</script>

<label class="label mb-4 flex flex-col justify-start space-y-2">
	<span class="text-sm text-secondary-800 dark:text-white">{label}</span>
	<input
		{name}
		{type}
		{value}
		{placeholder}
		class="dark:border-primary-950 input rounded border-2 text-black placeholder-primary-200 dark:bg-primary-800 dark:text-primary-50"
	/>
</label>
