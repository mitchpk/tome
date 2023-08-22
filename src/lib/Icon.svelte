<script lang="ts">
    export let name: string;
    export let size: string = "24";
</script>

<svg class="feather" style="width: {size}px; height: {size}px;">
    <use href={`/feather-sprite.svg#${name}`}/>
</svg>

<style lang="scss">
    .feather {
        stroke: currentColor;
        stroke-width: 2;
        stroke-linecap: round;
        stroke-linejoin: round;
        fill: none;
    }
</style>
