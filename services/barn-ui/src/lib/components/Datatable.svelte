<script lang="ts">
	import {
		type TableHandler,
		Datatable,
		RowsPerPage,
		ThSort,
		RowCount,
		Pagination
	} from '@vincjo/datatables/server';
	let { table }: { table: TableHandler } = $props();
</script>

<Datatable {table}>
	{#snippet header()}
		<div></div>
		<RowsPerPage {table} />
	{/snippet}
	<table>
		<thead>
			<tr>
				<ThSort {table} field="id">ID</ThSort>
				<ThSort {table} field="name">Name</ThSort>
				<ThSort {table} field="email">Email</ThSort>
				<ThSort {table} field="body">Comment</ThSort>
			</tr>
		</thead>
		<tbody>
			{#each table.rows as row}
				<tr>
					<td><b>{row.title}</b></td>
				</tr>
			{/each}
		</tbody>
	</table>
	{#snippet footer()}
		<RowCount {table} />
		<Pagination {table} />
	{/snippet}
</Datatable>
