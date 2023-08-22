<script lang="ts">
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import type { Album } from "src/bindings/Album";
    import { albums, selectedAlbum } from "../store";
    import Icon from "./Icon.svelte";

    const toggleSelect = (album: Album) => {
        if ($selectedAlbum?.id == album.id)
            selectedAlbum.set(null);
        else {
            selectedAlbum.set(album);
        }
    }
</script>

<div id="album-view">
    <div id="album-header">
        <input type="text" placeholder="Search" style="width: 20em;">
        <button class="small">
            <Icon name="list" size="18"/>
            &nbsp;Sort: Name
        </button>
    </div>
    <div id="album-grid">
        { #each $albums as album }
            <div class="album" on:click={() => toggleSelect(album)}>
                <img src={convertFileSrc(album.artwork_path)} alt="Track art">
                <div class="details">
                    <span class="title">{album.title}</span><br>
                    <span class="artist">{album.artist}</span>
                </div>
            </div>
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
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
        gap: 16px;
        padding: 16px;
        //padding-top: 0;
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
