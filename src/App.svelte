<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import { appWindow } from '@tauri-apps/api/window';
    import type { UnlistenFn } from '@tauri-apps/api/event';
    import Sidebar from './lib/Sidebar.svelte';
    import ControlBar from './lib/ControlBar.svelte';
    import AlbumView from './lib/AlbumView.svelte';
    import AlbumDetails from './lib/AlbumDetails.svelte';
    import Database from './database';

    let unlistenFileDrop: UnlistenFn;

    onMount(async () => {
        await Database.initialise();
        unlistenFileDrop = await appWindow.onFileDropEvent((evt) => {
            switch (evt.payload.type) {

            }
        })
    });

    onDestroy(() => {
        unlistenFileDrop();
    });
</script>

<main id="container">
    <div class="row" style="flex: 1; min-height: 0; overflow: hidden;">
        <Sidebar />
        <AlbumView />
        <AlbumDetails />
    </div>
    <ControlBar />
</main>

<style lang="scss">
    #container {
        display: flex;
        flex-direction: column;
        height: 100vh;
    }
</style>
