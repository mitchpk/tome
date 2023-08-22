<script lang="ts">
    import Icon from "./Icon.svelte";

    export let headerText: string;
    let expanded = false;
</script>

<div class="collapsible">
    <button class="header small" aria-expanded={expanded} on:click={() => expanded = !expanded}>
        <div hidden={expanded}>
            <Icon name="chevron-down" size="18" />
        </div>
        <div hidden={!expanded}>
            <Icon name="chevron-up" size="18" />
        </div>
        {headerText}
    </button>
    <div class="contents" hidden={!expanded}>
        <slot></slot>
    </div>
</div>

<style lang="scss">
    .collapsible {
        display: flex;
        flex-direction: column;
    }

    .contents {
        //border-bottom: 1px solid var(--background-tertiary);
    }

    .header {
        color: var(--blue);
        text-align: left;
        text-transform: uppercase;
    }

    .header:hover {
        background-color: transparent;
        color: var(--foreground-primary);
    }
</style>
