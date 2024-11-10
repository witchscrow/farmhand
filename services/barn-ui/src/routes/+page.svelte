<script>
	import FeatureCard from '$lib/components/FeatureCard.svelte';
</script>

<section class="my-10 flex flex-col space-y-8 text-center">
	<div class="space-y-4">
		<h1 class="font-serif text-4xl text-secondary-700 dark:text-primary-500">Farmhand</h1>
		<p class="text-lg text-secondary-800 dark:text-primary-100">It's farming time, yeehaw! 🌾</p>
		<p class="mx-auto max-w-2xl text-black dark:text-white">
			A powerful, open-source clip and VOD management system built for creators and artists who want
			more control over their content.
		</p>
	</div>

	<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
		<FeatureCard
			title="Advanced Clip Editor"
			description="Create perfect moments with our Twitch-style clip editor, giving you precise control over your highlights."
			icon="✂️"
		/>
		<FeatureCard
			title="Smart Backlinking"
			description="Jump straight to the source with clip backlinking, letting viewers watch the full VOD from any clip's timestamp."
			icon="🔗"
		/>
		<FeatureCard
			title="Intelligent Analysis"
			description="Automatically detect clip-worthy moments through advanced video and audio analysis."
			icon="🤖"
		/>
		<FeatureCard
			title="Chat-Aware Tagging"
			description="Automatically tag clips based on chat activity and reactions, making content discovery a breeze."
			icon="🏷️"
		/>
		<FeatureCard
			title="Smart Filtering"
			description="Find the perfect clip with flexible filtering based on automatic tags like 'exciting' or 'Pog'."
			icon="🔍"
		/>
		<FeatureCard
			title="Open Source"
			description="Free and open source forever. Built by creators, for creators."
			icon="💚"
		/>
	</div>
</section>
