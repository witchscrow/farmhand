<script>
	import { ProgressRadial } from '@skeletonlabs/skeleton';

	let { width = 'w-10', stroke = 100 } = $props();
</script>

<ProgressRadial
	meter="stroke-primary-500"
	track="stroke-primary-500/30"
	strokeLinecap="round"
	{width}
	{stroke}
/>
