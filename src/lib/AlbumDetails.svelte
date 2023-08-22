<script lang="ts">
    import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
    import player from "../player";
    import { type Track } from "../bindings/Track";
    import { selectedAlbum, currentTrack, isPlaying, isLoading } from "../store";
    import Icon from "./Icon.svelte";

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
</script>

{#if $selectedAlbum}
    <div id="album-details">
        <img id="background" src={convertFileSrc($selectedAlbum.artwork_path || "")}>
        <img id="art" src={convertFileSrc($selectedAlbum.artwork_path || "")}>
        <div id="contents">
            <div id="tracks">
                {#each tracks as track, id}
                    <button on:click={() => clickTrack(id)}>
                        <div hidden={$currentTrack?.id == track.id && $isPlaying}>
                            <Icon name="play" size="16" />
                        </div>
                        <div hidden={$currentTrack?.id != track.id || !$isPlaying}>
                            <Icon name="pause" size="16" />
                        </div>
                        <span>{track.metadata.track_number}:</span>
                        <span>{track.metadata.title}</span>
                        <span class="spacer" />
                        {formatTime(track.duration)}
                    </button>
                {/each}
            </div>
        </div>
    </div>
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

    button {
        text-align: left;
        gap: 4px;
    }
</style>
