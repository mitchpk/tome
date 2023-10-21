<script lang="ts">
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import type { Album } from "src/bindings/Album";
    import { albums, selectedAlbum } from "../store";
    import Icon from "./Icon.svelte";
    import AlbumDropdown from "./AlbumDropdown.svelte";

    const toggleSelect = (album: Album) => {
        if ($selectedAlbum?.id == album.id)
            selectedAlbum.set(null);
        else {
            selectedAlbum.set(album);
        }
    }

    let width: number = 1;
    $: chunkSize = Math.ceil(width / 200);
    $: rows = [...Array(Math.ceil($albums.length / chunkSize))].map((_, i) => {
        let chunk: (Album | null)[] = $albums.slice(i * chunkSize, (i + 1) * chunkSize);
        while (chunk.length < chunkSize) {
            chunk.push(null);
        }
        return chunk;
    });
</script>

<div id="album-view">
    <div id="album-header">
        <input type="text" placeholder="Search" style="width: 320px;">
        <button class="small">
            <Icon name="list" size="18"/>
            &nbsp;Sort: Name
        </button>
    </div>
    <div id="album-grid" bind:clientWidth={width}>
        <!-- { #each $albums as album }
            <div class="album" on:click={() => toggleSelect(album)}>
                <img src={convertFileSrc(album.artwork_path)} alt="Track art">
                <div class="details">
                    <span class="title">{album.title}</span><br>
                    <span class="artist">{album.artist}</span>
                </div>
            </div>
        { /each } -->
        { #each rows as row }
            <div class="row">
                { #each row as album }
                    { #if album }
                        <div class="album" on:click={() => toggleSelect(album)}>
                            <img src={convertFileSrc(album.artwork_path)} alt="Track art">
                            <div class="details">
                                <span class="title">{album.title}</span><br>
                                <span class="artist">{album.artist}</span>
                            </div>
                        </div>
                    { :else }
                        <div style="flex: 1"/>
                    { /if }
                { /each }
            </div>
            { @const selected = $selectedAlbum != null ? row.indexOf($selectedAlbum) : -1 }
            { #if selected != -1 }
                <AlbumDropdown album={$selectedAlbum} />
            { /if }
        { /each }
    </div>
</div>

<style lang="scss">
    #album-view {
        flex: 1;
        display: flex;
        flex-direction: column;
        overflow-y: auto;
        border-left: 1px solid var(--background-tertiary);
        border-bottom: 1px solid var(--background-tertiary);
        background-color: var(--background-primary);
    }

    #album-grid {
        /*display: grid;
        grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));*/
        display: flex;
        flex-direction: column;
        gap: 16px;
        padding: 16px 0;
        position: inherit !important;
    }

    .row {
        display: flex;
        gap: 16px;
        padding: 0 16px;
    }

    #album-header {
        display: flex;
        justify-content: space-between;
        position: sticky;
        backdrop-filter: blur(10px);
        -webkit-backdrop-filter: blur(10px);
        background-color: rgba(29, 31, 33, 0.8);
        z-index: 1;
        top: 0;
        padding: 8px 16px;
        border-bottom: 1px solid var(--background-tertiary);
        //box-shadow: 0px 0px 10px 5px rgba(0, 0, 0, 0.2);
    }

    .album {
        flex: 1;
        min-width: 0;
        cursor: pointer;
        &:hover img {
            transform: translateY(-10px);
            box-shadow: 0px 10px 20px 0px rgba(0, 0, 0, 0.5);
        }

        .details {
            margin-top: 8px;
            text-overflow: ellipsis;
            overflow: hidden;

            .title {
                white-space: nowrap;
            }

            .artist {
                color: var(--grey);
            }
        }

        img {
            width: 100%;
            object-fit: cover;
            aspect-ratio: 1;
            border-radius: var(--border-radius);
            box-shadow: 0px 2px 5px 0px rgba(0, 0, 0, 0.5);
            transition: all 0.5s cubic-bezier(.17,.67,.36,.94);
        }
    }
</style>
