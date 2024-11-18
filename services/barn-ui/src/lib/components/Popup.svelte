<script lang="ts">
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';

	let { action, content, id, onOpen } = $props();
	const popupClick: PopupSettings = {
		event: 'click',
		target: id,
		placement: 'top',
		state: (e) => onOpen(e.state)
	};
</script>

<div use:popup={popupClick}>
	{@render action()}
</div>
<div data-popup={id}>
	{@render content()}
</div>
