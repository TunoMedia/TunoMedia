<script lang="ts">
	let { musicPlayer, song, index } = $props();
</script>

<div class="group flex justify-between gap-x-6 py-5 rounded-3xl {musicPlayer.songPlayingIndex === index ? "bg-indigo-300" : "hover:bg-indigo-100"}">
    <div class="flex min-w-0 gap-x-4 ml-4">
        <div class="min-w-0 flex-auto">
            <p class="text-sm/6 font-semibold text-gray-900">{song.title}</p>
            <p class="mt-1 truncate text-xs/5 text-gray-500">{song.author}</p>
        </div>
    </div>
    <div>
    </div>
    <div class="{musicPlayer.songPlayingIndex === index ? "flex" : "hidden group-hover:flex"} min-w-0 gap-x-4 mr-4">
        <button aria-label="play" onclick={() => musicPlayer.togglePlayingSelectedSong(index)}>
            <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="#4f46e5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                {#if musicPlayer.isPlaying && musicPlayer.songPlayingIndex === index}
                    <rect x="6" y="4" width="4" height="16"></rect>
                    <rect x="14" y="4" width="4" height="16"></rect>
                {:else}
                    <polygon points="5 3 19 12 5 21 5 3"></polygon>
                {/if}
            </svg>
        </button>
    </div>

</div>
