import { getContext, setContext } from "svelte";
import { TunoSocket } from "./TunoSocket.svelte";

export type SongObject = {
    object_id: string,
    name: string,
    author: string,
}

export class MusicPlayer {
	#songs: SongObject[] = [];
    isPlaying: boolean = $state(false);
	songPlayingIndex = $state(0);

    #socket: TunoSocket = new TunoSocket("ws://localhost:4114");
    #audio: HTMLAudioElement = new Audio();
    #mediaSource: MediaSource = new MediaSource();
    #sourceBuffer: SourceBuffer | null = null;

    constructor() {
        this.#mediaSource.addEventListener('sourceopen', () => {
            this.#sourceBuffer = this.#mediaSource.addSourceBuffer('audio/mpeg');
        })

        this.#audio.src = URL.createObjectURL(this.#mediaSource);
    }

    addNewSong(song: SongObject) {
        if (this.#songs.push(song) === 1) this.loadSong()
    }

    loadSong() {
        this.#socket.stream(this.#songs[this.songPlayingIndex].object_id)
            .then(blob => blob.arrayBuffer())
            .then(buf => {
                if (!this.#sourceBuffer) return console.error("sourceBuffer do not exist");
                this.#sourceBuffer.appendBuffer(buf)
            })
            .catch(error => console.error(error));
    }

    togglePlaying() {
        this.isPlaying ? this.pause() : this.play()
    }

    pause() {
        this.isPlaying = false
        this.#audio.pause()
    }

    play() {
        this.isPlaying = true
        this.#audio.play()
    }

    togglePlayingSelectedSong(index: number) {
        if (this.songPlayingIndex != index) {
            this.songPlayingIndex = index
            this.isPlaying = false
            this.loadSong()
        }

        this.togglePlaying()
    }

    previous() {
		if (this.songPlayingIndex <= 0) return
		this.songPlayingIndex -= 1

        this.loadSong()
		this.play()
	}

	next() {
		if (this.songPlayingIndex >= this.#songs.length - 1) return
		this.songPlayingIndex += 1

        this.loadSong()
		this.play()
	}
}

const PLAYER_KEY = Symbol('PLAYER_KEY');

export function setMusicPlayerContext() {
    return setContext(PLAYER_KEY, new MusicPlayer());
}

export function getMusicPlayerContext() {
    return getContext<ReturnType<typeof setMusicPlayerContext>>(PLAYER_KEY);
}