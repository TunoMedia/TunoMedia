<script lang="ts">
    import { getMusicPlayerContext } from '$lib/Player/MusicPlayer.svelte';

    let { song, index } = $props();
    let player = getMusicPlayerContext();

    function msToHMS(duration: number): String {
        let seconds = Math.floor((duration / 1000) % 60);
        let minutes = Math.floor((duration / (1000 * 60)) % 60);
        let hours = Math.floor((duration / (1000 * 60 * 60)) % 24);

        let hours_display = (hours == 0) ? "" : `${hours}:`;
        let minutes_display = minutes.toString().padStart(2, "0") + ":";
        let seconds_display = seconds.toString().padStart(2, "0");

        return hours_display + minutes_display + seconds_display;
    }

    let selected = $derived(player.songPlayingIndex === index);
    let active = $derived(player.isPlaying && selected);
</script>

{#await song}
    <span>waiting...</span>
{:then fields} 
    <button
        class="group h-full w-full flex {selected ? "bg-black" : "bg-white hover:bg-black"} rounded-r-full drop-shadow-[0_3px_3px_rgba(0,0,0,0.25)] cursor-pointer"
        onclick={() => player.togglePlayingSelectedSong(index)}
    >
        <div class="h-full flex-none relative">
            <img src={fields.cover_art_url} class="h-full" alt="{fields.title}'s cover art"/>
                <div class="absolute inset-0 {active ? "bg-black" : "bg-white"} rounded-full p-3">
                    <svg width="100%" height="100%" viewBox="0 0 24 24" fill="{active ? "white" : "black"}">
                        {#if active}
                            <rect x="6" y="6" width="4" height="14" />
                            <rect x="14" y="6" width="4" height="14" />
                        {:else}
                            <polygon points="8,5 8,19 19,12"/>
                        {/if}   
                    </svg>
                </div>
        </div>
        <div class="grow px-2 min-w-0 ">
            <div class="h-full flex justify-between items-center">
                <div class="min-w-0 mr-2 text-left">
                    <p class="text-xs/4 font-semibold {selected ? "text-gray-100" : "text-gray-900 group-hover:text-gray-100"} truncate">{fields.title}</p>
                    <p class="text-xs/4 {selected ? "text-gray-400" : "text-gray-500 group-hover:text-gray-400"} truncate">{fields.artist}</p>
                </div>
                <div class="flex">
                    <span class="text-xs/5 m-auto {selected ? "text-gray-100" : "group-hover:text-gray-100"}">{msToHMS(fields.duration)}</span>
                </div>
            </div>
        </div>
        <div class="flex-none">
            <img 
                src={fields.cover_art_url} 
                class="h-full rounded-full album-cover {active ? 'playing' : ''}" 
                alt="{fields.title}'s cover art"
            />
        </div>
    </button>
{/await}

<style>
    @keyframes rotate {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }   
    }
    
    .album-cover {
        animation: rotate 5s linear infinite;
        animation-play-state: paused;
    }
    
    .album-cover.playing {
        animation-play-state: running;
    }
</style>
