CREATE TABLE track (
    id TEXT NOT NULL PRIMARY KEY,
    path TEXT NOT NULL,
    title TEXT NOT NULL,
    duration INTEGER NOT NULL,
    artist TEXT NOT NULL REFERENCES artist,
    album_id TEXT NOT NULL REFERENCES album,
    genre TEXT REFERENCES genre,
    song_artist TEXT,
    track_number INTEGER,
    cd_number INTEGER,
    year INTEGER,
    artwork_path TEXT
);

CREATE TABLE artist (
    name TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE album (
    id TEXT NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    artist TEXT NOT NULL REFERENCES artist,
    track_count INTEGER NOT NULL,
    artwork_path TEXT
);

CREATE TABLE genre (
    name TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE playlist_track (
    playlist TEXT NOT NULL REFERENCES playlist,
    track_id TEXT NOT NULL REFERENCES track,
    position INTEGER NOT NULL,
    PRIMARY KEY (playlist, track_id)
);

CREATE TABLE playlist (
    title TEXT NOT NULL PRIMARY KEY
);
