<script lang="ts">
	import { setContext, createEventDispatcher } from 'svelte';
	import { fade } from 'svelte/transition';
	import { key } from '../menu.ts';

	export let x;
	export let y;

	// whenever x and y is changed, restrict box to be within bounds
	$: (() => {
		if (!menuEl) return;

		const rect = menuEl.getBoundingClientRect();
		x = Math.min(window.innerWidth - rect.width, x);
		if (y > window.innerHeight - rect.height) y -= rect.height;
	})(x, y);

	const dispatch = createEventDispatcher();

	setContext(key, {
		dispatchClick: () => dispatch('click')
	});

	let menuEl;
	function onPageClick(e: MouseEvent) {
		if (e.target === menuEl || menuEl.contains(e.target)) return;
		dispatch('clickoutside');
	}
</script>

<style lang="scss">
	div {
		position: absolute;
		display: grid;
		border: 1px solid var(--background-tertiary);
		box-shadow: 2px 2px 5px 0px #0002;
		background: var(--background-secondary);
        z-index: 999;
	}
</style>

<svelte:body on:click={onPageClick} />

<div
    transition:fade={{ duration: 100 }}
    bind:this={menuEl}
    style="top: {y}px; left: {x}px;"
    on:contextmenu|preventDefault
>
	<slot />
</div>
