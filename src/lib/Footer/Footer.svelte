<script lang="ts">
    import { getMusicPlayerContext } from "$lib/Player/MusicPlayer.svelte";

    let player = getMusicPlayerContext();
</script>

<div class="flex flex-col justify-center gap-6 sm:flex-row items-center">
    <div class="flex items-center">
        <div class="flex space-x-3 p-2">
            <button class="focus:outline-none" aria-label="previous" onclick={() => player.previous()}>
                <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="#4f46e5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="19 20 9 12 19 4 19 20"></polygon><line x1="5" y1="19" x2="5" y2="5"></line></svg>
            </button>
            
            <button class="focus:outline-none" aria-label="play" onclick={() => player.togglePlaying()}>
                <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="#4f46e5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    {#if player.isPlaying}
                        <rect x="6" y="4" width="4" height="16"></rect>
                        <rect x="14" y="4" width="4" height="16"></rect>
                    {:else}
                        <polygon points="5 3 19 12 5 21 5 3"></polygon>
                    {/if}
                </svg>
            </button>

            <button class="focus:outline-none" aria-label="next" onclick={() => player.next()}>
                <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="#4f46e5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="5 4 15 12 5 20 5 4"></polygon><line x1="19" y1="5" x2="19" y2="19"></line></svg>
            </button>
        </div>
    </div>

    <div class="relative w-4/6 sm:w-1/2 md:w-7/12 ml-2">
        <div class="bg-indigo-300 h-2 w-full rounded-lg"></div>
        <div class="bg-indigo-600 h-2 w-1/2 rounded-lg absolute top-0"></div>
    </div>
</div>