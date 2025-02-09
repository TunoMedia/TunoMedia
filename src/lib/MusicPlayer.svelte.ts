type SongObject = {
    id: string,
    title: string,
    author: string,
}

export class MusicPlayer {
	songs: SongObject[] = $state([]) as SongObject[];
    isPlaying: boolean = $state(false);
	songPlayingIndex = $state(0);
    #audio: HTMLAudioElement = $state(new Audio());

    constructor(songs: SongObject[]) {
        this.songs = songs;
        this.loadSong()
    }

    loadSong() {
        this.#audio.src = `/${this.songs[this.songPlayingIndex].id}.mp3`;
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
		if (this.songPlayingIndex >= this.songs.length - 1) return
		this.songPlayingIndex += 1

        this.loadSong()
		this.play()
	}
}
