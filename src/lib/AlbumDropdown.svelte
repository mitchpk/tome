<script lang="ts">
    import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
    import { open } from "@tauri-apps/api/dialog";
    import player from "../player";
    import { type Track } from "../bindings/Track";
    import { currentTrack, isPlaying, isLoading } from "../store";
    import Icon from "./Icon.svelte";
    import Menu from "./Menu.svelte";
    import MenuOption from "./MenuOption.svelte";
    import type { Album } from "src/bindings/Album";

    export let album: Album;

    let tracks: Track[] = [];
    $: {
        tracks = [];
        invoke<Track[]>("get_tracks", { album: album.id })
            .then(res => {
                tracks = res.sort(
                    (a, b) => (a.metadata.track_number || 0) - (b.metadata.track_number || 0)
                )
            });
    }

    let showMenu = false;
    let pos = { x: 0, y: 0 };
    let selected = 0;
    let colour = [0, 0, 0];

    $: dark = (colour[0] * 0.299 + colour[1] * 0.587 + colour[2] * 0.114) > 150;
    $: gradient = `linear-gradient(180deg, rgb(${colour}), rgba(${colour},0.8) 100%)`

    const getColor = (e) => {
        let image = e.target;
        //colour = colorthief.getColor(image);
    }

    const formatTime = (seconds: number) => {
        const h = Math.floor(seconds / 3600);
        const m = Math.floor((seconds % 3600) / 60);
        const s = Math.round(seconds % 60);
        return [
            h,
            m > 9 ? m : (h ? '0' + m : m || '0'),
            s > 9 ? s : '0' + s
        ].filter(Boolean).join(':');
    }

    const clickTrack = async (index: number) => {
        if ($isLoading) return;
        if ($currentTrack?.id == tracks[index].id) {
            if ($isPlaying) {
                await player.pause();
            } else {
                await player.resume();
            }
        } else {
            player.queue = [];
            player.position = 0;
            for (let track of tracks) {
                player.queueTrack(track);
            }
            player.position = index;
            await player.playNext();
        }
    }

    const rightClick = async (e: MouseEvent, index: number) => {
        if (showMenu) {
            showMenu = false;
            await new Promise(res => setTimeout(res, 100));
        }

        selected = index;
        pos = { x: e.clientX, y: e.clientY };
        showMenu = true;
    }

    const extractStems = async () => {
        const outDir = await open({
            directory: true,
            multiple: false
        });
        if (outDir == null) {
            return;
        }

        invoke("run_demucs", {
            track: tracks[selected],
            outDir
        })
    }
</script>

<div id="album-details" class:dark={dark}>
    <img id="background-art" src={convertFileSrc(album.artwork_path || "")}>
    <div id="contents">
        <div id="tracks">
            {#each tracks as track, id}
                <button on:click={() => clickTrack(id)} on:contextmenu|preventDefault={e => rightClick(e, id)}>
                    <div hidden={$currentTrack?.id == track.id && $isPlaying}>
                        <Icon name="play" size="16" />
                    </div>
                    <div hidden={$currentTrack?.id != track.id || !$isPlaying}>
                        <Icon name="pause" size="16" />
                    </div>
                    <span>{track.metadata.track_number}:</span>
                    <span class="title">{track.metadata.title}</span>
                    <span>{formatTime(track.duration)}</span>
                </button>
            {/each}
        </div>
    </div>
</div>
{#if showMenu}
    <Menu {...pos} on:click={() => showMenu = false} on:clickoutside={() => showMenu = false}>
        <MenuOption text="Show metadata" icon="search" />
        <MenuOption text="Extract stems" icon="music" on:click={extractStems} />
    </Menu>
{/if}

<style lang="scss">
    #album-details {
        flex: 1;
        overflow: hidden;
        box-shadow: 0px 2px 10px 0px rgba(0, 0, 0, 0.5);
        display: flex;
        min-height: 0;
        height: 400px;
        border: 1px solid var(--background-tertiary);
        border-left: none;
        border-right: none;
        position: relative;
        z-index: 0;
    }

    #art {
        object-fit: cover;
        aspect-ratio: 1;
        min-height: 100%;
        height: 0;
        transition: opacity 0.5s;
    }

    #contents {
        padding: 14px;
        transition: opacity 0.5s 0.5s;
        flex: 1;
        height: 100%;
    }

    #background-art {
        position: absolute;
        object-fit: cover;
        aspect-ratio: 1;
        width: 100%;
        height: 100%;
        filter: blur(100px) saturate(1.2);
        transform: scale3d(1.2, 1.2, 1);
        z-index: -1;
    }

    #tracks {
        display: flex;
        flex-direction: column;
    }

    .title {
        white-space: nowrap;
        text-overflow: ellipsis;
        flex: 1;
        overflow: hidden;
    }

    button {
        text-align: left;
        gap: 4px;
    }

    #album-details.dark  {
        background-color: white;
    }

    .dark button {
        color: var(--background-primary);
    }
</style>
