<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import { isPlaying, volume, currentTrack, currentTime, isMuted, isArtExpanded } from "../store";
    import Icon from "./Icon.svelte";
    import player from "../player";
    import { get } from "svelte/store";
    import { convertFileSrc } from "@tauri-apps/api/tauri";

    async function chooseFile() {
        let path = await open();
        if (Array.isArray(path)) return;
        console.log(path);
        if (!path) return;
        await player.playTrack({
            id: "",
            path,
            metadata: {
                title: "",
                artist: "",
                song_artist: "",
                album: "",
                track_number: 0,
                cd_number: 0,
                year: null,
                genre: null,
                artwork_path: null,
            },
            duration: 0,
        });
    }

    async function playPause() {
        if ($isPlaying) {
            await player.pause();
        } else {
            await player.resume();
        }
    }

    function changeVolume(event: Event) {
        const target = event.target as HTMLInputElement;
        volume.set(parseFloat(target.value));
    }
</script>

<div id="control-bar">
    <div class="row">
        {#if $currentTrack && !$isArtExpanded}
            <img id="track-art" alt="Track art" on:click={() => isArtExpanded.update(v => !v)} src={convertFileSrc($currentTrack?.metadata.artwork_path || "")}>
        {/if}
        {$currentTrack?.metadata.title}
        {$currentTrack?.metadata.artist}
        {$currentTime}
    </div>
    <div class="row">
        <div id="media-controls">
            <button on:click={async () => await player.playPrev()}>
                <Icon name="skip-back"/>
            </button>

            <button on:click={playPause}>
                <div hidden={$isPlaying}>
                    <Icon name="play" size="32" />
                </div>
                <div hidden={!$isPlaying}>
                    <Icon name="pause" size="32" />
                </div>
            </button>

            <button on:click={async () => await player.playNext()}>
                <Icon name="skip-forward"/>
            </button>

            <button on:click={chooseFile}>
                <Icon name="file"/>
            </button>
        </div>
    </div>
    <div class="row">
        <div id="volume">
            <button on:click={() => isMuted.update(v => !v)}>
                <span hidden={$volume >= 0.33 || $isMuted}>
                    <Icon name="volume"/>
                </span>
                <span hidden={$volume < 0.33 || $volume >= 0.67 || $isMuted}>
                    <Icon name="volume-1"/>
                </span>
                <span hidden={$volume < 0.67 || $isMuted}>
                    <Icon name="volume-2"/>
                </span>
                <span hidden={!$isMuted}>
                    <Icon name="volume-x"/>
                </span>
            </button>
            <input type="range" on:input={changeVolume} min="0" max="1" step="0.05" value={get(volume)} hidden={$isMuted}>
        </div>
        <button>
            <Icon name="more-horizontal"/>
        </button>
    </div>
</div>

<style lang="scss">
    #track-art {
        width: 48px;
        height: 48px;
        aspect-ratio: 1;
        object-fit: cover;
    }

    #control-bar {
        display: flex;
        padding: 8px;

        div {
            flex: 1;
        }
    }

    #media-controls {
        display: flex;
        align-items: center;
        margin-left: 8px;
        margin-right: 32px;

        button {
            padding: 10px;
            border-radius: 2px;
        }
    }

    #volume {
        display: flex;
        align-items: center;

        input {
            -webkit-appearance: none;
            appearance: none;
            height: 4px;
            background: var(--background-tertiary);
            outline: none;
            opacity: 1;
            border-radius: 2px;
            -webkit-transition: 0.2s;
            transition: opacity 0.2s;
            padding: 0;

            &::-webkit-slider-thumb {
                -webkit-appearance: none;
                appearance: none;
                width: 12px;
                height: 12px;
                border-radius: 50%;
                background-color: var(--foreground-primary);
            }
        }
    }
</style>
