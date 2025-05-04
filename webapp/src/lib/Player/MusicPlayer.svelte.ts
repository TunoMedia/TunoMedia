import { invoke } from '@tauri-apps/api/core';

import { getContext, setContext } from "svelte";
import { TunoSocket } from "./TunoSocket.svelte";

export type SongObject = {
    object_id: string,
    name: string,
    author: string,
}

export class MusicPlayer {
	#songs: any[] = $state([]);
    isPlaying: boolean = $state(false);
	songPlayingIndex = $state(0);
    activeSong = $derived(this.#songs[this.songPlayingIndex])

    #socket: TunoSocket = new TunoSocket();
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

        this.#mediaSource.addEventListener("sourceopen", () => {
            console.log("mse: sourceopen")
            this.#sourceBuffer = this.#mediaSource.addSourceBuffer('audio/mpeg');
            this.#sourceBuffer.onerror = (err) => {
                console.error("SOURCE BUFFER err:", err);
            }
        });

        this.#audio.src = URL.createObjectURL(this.#mediaSource);
    }

    #resetMediaSource() {
        for (let buffer of this.#mediaSource.activeSourceBuffers) {
            this.#mediaSource.removeSourceBuffer(buffer)
        }

        if (this.#mediaSource.readyState === 'ended') {
            this.#audio.src = URL.createObjectURL(this.#mediaSource);
        }
    }

    async addNewSong(song: SongObject) {
        if (this.#songs.push(song) === 1) await this.loadSong()
    }

    async loadSong() {
        this.#resetMediaSource();

        let songId = this.activeSong.objectId;
        let resp = await invoke("get_distributor", { songId })
            .catch((error) => console.error(error));

        if (!resp) return
        let [ url, tx ] = resp as [string, string];

        this.#socket = new TunoSocket(url)

        try {
            let streamCall = this.#socket.streamSong(tx);

            let index = 0;
            for await (let buf of streamCall) {
                if (!this.#sourceBuffer) return console.error("sourceBuffer do not exist");

                let update = new Promise(res => this.#sourceBuffer!.addEventListener("updateend", res));
                this.#sourceBuffer.appendBuffer(buf)

                await update;
                index++;
            }

            this.#mediaSource.endOfStream();
            URL.revokeObjectURL(this.#audio.src);
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

        this.loadSong()
		this.play()
	}

	async next() {
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