<script lang="ts">
    import { getContext } from 'svelte';
    import { key } from '../menu.ts';

    export let isDisabled = false;
    export let text = "";
    export let icon: string | null = null;

    import { createEventDispatcher } from 'svelte';
    import Icon from './Icon.svelte';
    const dispatch = createEventDispatcher();

    const { dispatchClick } = getContext(key);

    const handleClick = e => {
        if (isDisabled) return;

        dispatch('click');
        dispatchClick();
    }
</script>

<style>
    button {
        padding: 4px 6px;
        cursor: default;
        display: flex;
        gap: 5px;
    }
</style>

<button
    class="small"
    class:disabled={isDisabled}
    on:click={handleClick}
>
    {#if text}
        {#if icon}
            <Icon name={icon} size="16" />
        {:else}
            <span style="width: 16px"></span>
        {/if}
        {text}
    {:else}
    <slot />
    {/if}
</button>

