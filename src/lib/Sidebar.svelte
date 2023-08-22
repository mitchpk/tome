<script lang="ts">
    import { convertFileSrc, invoke } from "@tauri-apps/api/tauri"
    import { type Artist } from "../bindings/Artist";
    import { onMount } from "svelte";
    import { currentTrack, selectedArtist, isArtExpanded } from "../store";
    import Collapsible from "./Collapsible.svelte";
    import type { Playlist } from "src/bindings/Playlist";
    import Icon from "./Icon.svelte";

    let artists: Artist[] = [];
    let playlists: Playlist[] = [];

    onMount(async () => {
        artists = await invoke("get_artists");
        artists.sort((a, b) => a.name.localeCompare(b.name));
        playlists = await invoke("get_playlists");
        playlists.sort((a, b) => a.title.localeCompare(b.title));
    })

    const newPlaylist = () => {

    }
</script>

<div id="sidebar">
    <div id="contents">
        <Collapsible headerText="Artists">
            <div class="list">
                <button
                    class:selected={$selectedArtist == null}
                    on:click={() => selectedArtist.set(null)}
                >All artists</button>
                {#each artists as artist}
                    <button
                        class:selected={artist.name == $selectedArtist}
                        on:click={() => selectedArtist.set(artist.name)}
                    >{artist.name}</button>
                {/each}
            </div>
        </Collapsible>
        <Collapsible headerText="Playlists">
            <div class="list">
                {#each playlists as playlist}
                    <button on:click={() => selectedArtist.set(playlist.title)}>{playlist.title}</button>
                {/each}
            </div>
        </Collapsible>
        <button class="small" on:click={newPlaylist}>
            <Icon name="plus" size="18" />
            &nbsp;New playlist
        </button>
    </div>
    {#if $currentTrack && $isArtExpanded}
        <img id="track-art" alt="Track art" on:click={() => isArtExpanded.update(v => !v)} src={convertFileSrc($currentTrack?.metadata.artwork_path || "")}>
    {/if}
</div>

<style lang="scss">
    #sidebar {
        width: 320px;
        display: flex;
        flex-direction: column;
    }

    #contents {
        flex: 1;
        overflow-y: auto;
        padding: 16px;
    }

    #track-art {
        box-shadow: 0px 2px 5px 0px rgba(0, 0, 0, 0.5);
        border-radius: var(--border-radius);
        z-index: 1;
        height: 288px;
        object-fit: cover;
        background-color: var(--background-primary);
        aspect-ratio: 1;
        margin: 16px;
        margin-top: 0;
    }

    button {
        color: var(--foreground-secondary);
        text-align: left;
    }

    .selected {
        color: var(--foreground-primary);
        background-color: var(--background-tertiary);
    }

    .list {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }
</style>
