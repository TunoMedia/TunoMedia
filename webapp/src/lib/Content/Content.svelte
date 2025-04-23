<script lang="ts">
    import AddSongButton from './AddSongButton.svelte';
    import SongListing from './SongListing.svelte';
    import { getMusicPlayerContext, type SongObject } from '$lib/Player/MusicPlayer.svelte';
    import { getWalletProviderContext } from '$lib/Wallet/WalletProviderContext.svelte';

    let player = getMusicPlayerContext();
    let wallet = getWalletProviderContext();

    const get_songs = async () => {
        if (!wallet.client) return

        let songs: String[] = [
            "7ca86b0d7f598698ecd6d94c5a474d41d122e5876c344f490667d12b9cbe6e63",
            "7463fa6bbbb2217298255e0dace21e2d00a8116b767fc55d371a08c51dd28573"
        ]
        
        return songs.map(async s => {
            let res = await fetch(`media/${s}`);
            let data = await res.json();

            player.addNewSong(data);

            return data.content.fields
        });
    }
</script>

<div class="h-full">
    <ul class="h-full">
        {#await get_songs() then songs }
            {#if songs}
                {#each songs as song, index}
                    <li class="pb-4 h-[7.5vh]">
                        <SongListing {song} {index} />
                    </li>
                {/each}
            {/if}
        {/await}
    </ul>
</div>
