<script lang="ts">
	let { class: className }: { class?: string } = $props();
</script>

<footer class="w-full py-8 text-black dark:text-white {className}">
	<div class="container mx-auto max-w-screen-lg px-4">
		<div class="mt-8 pt-8 text-center text-sm">
			<p>
				Made with <span class="text-red-500">❤</span> by
				<a
					class="font-semibold underline hover:text-primary-800"
					href="https://sneakycrow.dev"
					target="_blank"
				>
					sneakycrow
				</a>
			</p>
		</div>
	</div>
</footer>
