<script lang="ts">
	import type { Snippet } from 'svelte';

	let { columns, rows }: { columns: string[]; rows: Snippet } = $props();
</script>

<div class="table-container">
	<table class="table table-interactive table-hover table-compact">
		<thead>
			<tr>
				{#each columns as column}
					<th>{column}</th>
				{/each}
			</tr>
		</thead>
		<tbody>
			{@render rows()}
		</tbody>
	</table>
</div>
