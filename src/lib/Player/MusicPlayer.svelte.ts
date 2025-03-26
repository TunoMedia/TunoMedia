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

    #socket: TunoSocket = new TunoSocket("http://localhost:4114");
    #audio: HTMLAudioElement;
    #mediaSource: MediaSource;
    #sourceBuffer: SourceBuffer | null = null;

    constructor() {
        this.#audio = new Audio();
        this.#mediaSource = new MediaSource();

        if (!MediaSource.isTypeSupported("audio/mpeg")) {
            console.error("audio/mpeg not supported")
            return
        }

        // this.#mediaSource.addEventListener("sourceopen", () => this.#resetMediaSource());
        this.#audio.src = URL.createObjectURL(this.#mediaSource);
    }

    #resetMediaSource() {
        for (let buffer of this.#mediaSource.activeSourceBuffers) {
            this.#mediaSource.removeSourceBuffer(buffer)
        }

        this.#sourceBuffer = this.#mediaSource.addSourceBuffer('audio/mpeg');
        this.#sourceBuffer.onerror = (err) => {
            console.error("SOURCE BUFFER err:", err);
        }
    }

    async addNewSong(song: SongObject) {
        if (this.#songs.push(song) === 1) await this.loadSong()
    }

    async loadSong() {
        this.#resetMediaSource();

        try {
            let streamCall = this.#socket.streamSong(this.#songs[this.songPlayingIndex].object_id);

            let index = 0;
            for await (let buf of streamCall) {
                if (!this.#sourceBuffer) return console.error("sourceBuffer do not exist");

                if (this.#sourceBuffer.updating) {
                    await new Promise(res => this.#sourceBuffer!.addEventListener("updateend", res));
                }

                let appendTime = index > 0 ? this.#sourceBuffer.buffered.end(0) : 0;

                this.#sourceBuffer.appendWindowStart = appendTime;
                // this.#sourceBuffer.appendWindowEnd = appendTime + gaplessMetadata.audioDuration;

                // this.#sourceBuffer.timestampOffset = appendTime - gaplessMetadata.frontPaddingDuration;
                this.#sourceBuffer.timestampOffset = appendTime

                this.#sourceBuffer.appendBuffer(buf)
            }

            this.#mediaSource.endOfStream();
            // URL.revokeObjectURL(this.#audio.src);
        } catch(error) {
            console.error(error)
        }
    }

    togglePlaying() {
        this.isPlaying ? this.pause() : this.play()
    }

    pause() {
        this.isPlaying = false
        this.#audio.pause()
    }

    play() {
        console.log("start playing")
        this.isPlaying = true
        this.#audio.play()
    }

    async togglePlayingSelectedSong(index: number) {
        if (this.songPlayingIndex != index) {
            this.songPlayingIndex = index
            this.isPlaying = false
            await this.loadSong()
        }

        this.togglePlaying()
    }

    async previous() {
		if (this.songPlayingIndex <= 0) return
		this.songPlayingIndex -= 1

        await this.loadSong()
		this.play()
	}

	async next() {
		if (this.songPlayingIndex >= this.#songs.length - 1) return
		this.songPlayingIndex += 1

        await this.loadSong()
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