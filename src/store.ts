import { invoke } from "@tauri-apps/api";
import { writable, derived, type Writable } from "svelte/store";
import { tweened } from "svelte/motion";
import type { Album } from "./bindings/Album";
import type { Track } from "./bindings/Track";

export const currentTime = writable(0);
export const currentTrack: Writable<Track | null> = writable(null);
export const playlist: Writable<Track[]> = writable([]);
export const isPlaying = writable(false);
export const isLoading = writable(false);

export const volume = tweened(
    parseFloat(localStorage.getItem("volume") || "0.8"),
    { duration: 200 }
);
volume.subscribe((vol) => {
    localStorage.setItem("volume", vol.toString());
});

export const isMuted = writable(false);

export const selectedArtist: Writable<string | null> = writable(null);
export const selectedAlbum: Writable<Album | null> = writable(null);
export const albums = derived<Writable<string | null>, Album[]>(selectedArtist, ($selectedArtist, set) => {
    invoke<Album[]>("get_albums", { artist: $selectedArtist })
        .then(as => set(as.sort((a, b) => a.title.localeCompare(b.title))));
    return () => { set = () => {} };
}, []);

export const isArtExpanded = writable((localStorage.getItem("isArtExpanded") || "true") == "true");
isArtExpanded.subscribe((v) => {
    localStorage.setItem("isArtExpanded", v.toString());
})
