import { invoke } from '@tauri-apps/api';
import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
import { writable, type Writable } from 'svelte/store';
import type { Album } from './bindings/Album';
import type { Artist } from './bindings/Artist';
import type { Genre } from './bindings/Genre';
import type { Playlist } from './bindings/Playlist';
import type { Track } from './bindings/Track';

interface Query {
    where(predicate: string): Query;
}

class Database {
    tracks: Writable<Track[]>;
    artists: Writable<Artist[]>;
    albums: Writable<Album[]>;
    genres: Writable<Genre[]>;
    playlists: Writable<Playlist[]>;

    unlistenDb: UnlistenFn;

    async initialise() {
        this.unlistenDb = await listen<Track[]>("track_update", e => {
            this.tracks.set(e.payload);
        });
    }
}

export default new Database();
