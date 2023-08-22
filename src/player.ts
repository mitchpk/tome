import type { Track } from "./bindings/Track";
import { readBinaryFile } from "@tauri-apps/api/fs";
import { isPlaying, volume, currentTrack, currentTime, isMuted, isLoading } from "./store";
import { get } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

function perceivedLoudness(volume: number) {
    const a = 0.01; //0.001;
    const b = 4.606; //6.908;

    if (volume < 0.1)
        return volume * 10 * a * Math.exp(0.1 * b);

    return Math.min(a * Math.exp(b * volume), 1);
}

class Player {
    audioFile: HTMLAudioElement;
    queue: Track[];
    position: number;

    constructor() {
        this.audioFile = new Audio();
        this.audioFile.src = "";
        this.audioFile.addEventListener("ended", async () => {
            await this.playNext();
        });
        this.audioFile.volume = perceivedLoudness(get(volume));
        volume.subscribe((vol) => {
            this.audioFile.volume = perceivedLoudness(vol);
        });
        isMuted.subscribe((mute) => {
            this.audioFile.muted = mute;
        });
        //currentTrack.subscribe(this.updateControls.bind(this));
        //isPlaying.subscribe(this.updateControls.bind(this));
        setInterval(async () => {
            if (!this.audioFile.paused)
                currentTime.set(this.audioFile.currentTime);
        }, 100);
        setInterval(() => {
            if (get(isPlaying)) {
                this.updateControls();
            }
        }, 10000);
        listen("pause", _ => this.pause());
        listen("play", _ => this.resume());
    }

    async updateControls() {
        let track = get(currentTrack);
        let playing = get(isPlaying);
        invoke("update_controls", {
            track,
            playing,
            progress: currentTrack ? this.audioFile.currentTime : null
        });
    }

    getCurrentAudioFile() {
        return this.audioFile;
    }

    async playNext() {
        if (this.queue[this.position]) {
            await this.playTrack(this.queue[this.position]);
            this.position += 1;
        }
    }

    async playPrev() {
        if (this.queue[this.position - 2]) {
            this.position -= 1;
            await this.playTrack(this.queue[this.position - 1]);
        }
    }

    async playTrack(track: Track) {
        isLoading.set(true);
        isPlaying.set(true);
        currentTime.set(0);
        currentTrack.set(track);
        console.log(track);
        this.audioFile.pause();
        let file = await readBinaryFile(track.path);
        this.audioFile.src = URL.createObjectURL(new Blob([file]));
        await this.audioFile.play();
        this.audioFile.volume = perceivedLoudness(get(volume));
        isLoading.set(false);
        this.updateControls();
    }

    queueTrack(track: Track) {
        this.queue.push(track);
    }

    async resume() {
        if (get(isLoading)) return;
        this.audioFile.volume = perceivedLoudness(get(volume));
        await this.audioFile.play();
        isPlaying.set(true);
        this.updateControls();
    }

    async pause() {
        if (get(isLoading)) return;
        this.audioFile.pause();
        isPlaying.set(false);
        this.updateControls();
    }
}

export default new Player();
