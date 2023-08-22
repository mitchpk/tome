<script lang="ts">
    import Icon from "./Icon.svelte";

    export let loading = true;
</script>

{#if loading}
    <div class="loader">
        <Icon name="loader" />
    </div>
{:else}
    <slot></slot>
{/if}

<style lang="scss">
    .loader {
        margin: 0 auto;
        animation: rotating 1s infinite;
    }

    @keyframes rotating {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }
</style>
