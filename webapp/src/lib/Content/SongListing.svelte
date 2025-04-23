<script lang="ts">
    import { getMusicPlayerContext } from '$lib/Player/MusicPlayer.svelte';

    let { song, index } = $props();
    let player = getMusicPlayerContext();
</script>

{#await song}
    wait
{:then fields} 
    <div class="h-full flex bg-white rounded-r-full">
        <div class="h-full flex-none relative">
            <img src={fields.cover_art_url} class="h-full" alt="{fields.title}'s cover art"/>
            <div class="absolute inset-0 bg-white rounded-full p-3 cursor-pointer">
                <svg width="100%" height="100%" viewBox="0 0 24 24" fill="currentColor">
                    <polygon points="8,5 8,19 19,12" fill="black"/>
                </svg>
            </div>
        </div>
        <div class="grow px-2 min-w-0 ">
            <div class="h-full flex justify-between items-center">
                <div class="min-w-0 mr-2">
                    <p class="text-xs/4 font-semibold text-gray-900 truncate">{fields.title}</p>
                    <p class="text-xs/4 text-gray-500 truncate">{fields.artist}</p>
                </div>
                <div class="flex">
                    <span class="text-xs/5 m-auto">{fields.duration}</span>
                </div>
            </div>
        </div>
        <div class="flex-none">
            <img src={fields.cover_art_url} class="h-full rounded-full" alt="{fields.title}'s cover art"/>
        </div>
    </div>
{/await}
