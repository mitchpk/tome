<script lang="ts">
    import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
    import { open } from "@tauri-apps/api/dialog";
    import player from "../player";
    import { type Track } from "../bindings/Track";
    import { selectedAlbum, currentTrack, isPlaying, isLoading } from "../store";
    import Icon from "./Icon.svelte";
    import Menu from "./Menu.svelte";
    import MenuOption from "./MenuOption.svelte";

    let tracks: Track[] = [];
    $: {
        tracks = [];
        if ($selectedAlbum) {
            invoke<Track[]>("get_tracks", { album: $selectedAlbum.id })
                .then(
                    res => tracks = res.sort((a, b) => (a.metadata.track_number || 0) - (b.metadata.track_number || 0)
                    )
                );
        }
    }

    let showMenu = false;
    let pos = { x: 0, y: 0 };
    let selected = 0;

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

    const rightClick = async (e, index: number) => {
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

{#if $selectedAlbum}
    <div id="album-details">
        <img id="background" src={convertFileSrc($selectedAlbum.artwork_path || "")}>
        <img id="art" src={convertFileSrc($selectedAlbum.artwork_path || "")}>
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
{/if}

<style lang="scss">
    #album-details {
        width: 480px;
        background-color: black;
        overflow-y: auto;
        overflow-x: hidden;
        box-shadow: 0px 0px 20px 0px rgba(0, 0, 0, 0.5);
    }

    #art {
        width: 300px;
        margin: 90px auto;
        margin-bottom: 0;
        aspect-ratio: 1;
        object-fit: cover;
        box-shadow: 0px 2px 5px 0px rgba(0, 0, 0, 0.5);
        transition: opacity 0.5s;
    }

    #background {
        position: absolute;
        filter: blur(20px) brightness(0.5);
        aspect-ratio: 1;
        object-fit: cover;
        width: 520px;
        margin: -20px;
        transition: opacity 0.5s;
    }

    #contents {
        margin: 90px;
        margin-top: 16px;
        transition: opacity 0.5s 0.5s;
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
</style>
