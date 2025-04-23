<script lang="ts">
    import { getMusicPlayerContext } from "$lib/Player/MusicPlayer.svelte";

    let player = getMusicPlayerContext();
</script>

<div class="h-full relative overflow-y-hidden">
    <img class="w-full rounded-tl-[90px]" src="https://upload.wikimedia.org/wikipedia/en/4/42/Beatles_-_Abbey_Road.jpg" alt="Song's cover art" />
    <div class="absolute h-full inset-3 p-3">
        <div class="h-full rounded-t-full bg-black relative"></div>
    </div>

    <div class="absolute inset-0 p-3">
        <p class="text-white text-xs/4 font-semibold text-right">Abbey Road</p>
    </div>

    <div class="h-full w-full absolute inset-0">
        <div class="h-[27.5%] flex justify-center">
            <img src="https://static.lingq.com/media/resources/collections/images/2019/12/13/abb284a34d_ea2GDnV.jpg" class="h-full rounded-full" alt="Artist icon" />
        </div>

        <div>
            <div class="flex justify-center pt-1">
                <p class="text-white text-xs/4">The Beatles</p>
            </div>
            <div class="flex justify-center">
                <p class="text-white text-s/4 font-semibold">Here Comes the Sun</p>
            </div>

            <div class="flex justify-center gap-6 pt-2">
                <button class="focus:outline-none" aria-label="previous" onclick={() => player.previous()}>
                    <svg class="w-4 h-4" viewBox="0 0 24 24" fill="white" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="19 20 9 12 19 4 19 20"></polygon><line x1="5" y1="19" x2="5" y2="5"></line></svg>
                </button>

                <button class="focus:outline-none" aria-label="play" onclick={() => player.togglePlaying()}>
                    <svg class="w-6 h-6" viewBox="0 0 24 24" fill="white" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        {#if player.isPlaying}
                            <rect x="4" y="4" width="4" height="24"></rect>
                            <rect x="16" y="4" width="4" height="24"></rect>
                        {:else}
                            <polygon points="5 3 19 12 5 21 5 3"></polygon>
                        {/if}
                    </svg>
                </button>
    
                <button class="focus:outline-none" aria-label="next" onclick={() => player.next()}>
                    <svg class="w-4 h-4" viewBox="0 0 24 24" fill="white" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="5 4 15 12 5 20 5 4"></polygon><line x1="19" y1="5" x2="19" y2="19"></line></svg>
                </button>
            </div>

            <div class="flex justify-center pt-3">
                <div class="flex w-4/6 justify-center relative">
                    <div class="bg-white bg-opacity-10 h-2 w-full rounded-lg"></div>
                    <div class="bg-white h-2 w-1/2 rounded-lg absolute inset-0"></div>
                </div>
            </div>
        </div>
    </div>
</div>
