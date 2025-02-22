<script lang="ts">
    import AddSongButton from './AddSongButton.svelte';
    import SongListing from './SongListing.svelte';
    import { getMusicPlayerContext, type SongObject } from '$lib/MusicPlayer.svelte';
    import { getWalletProviderContext } from '$lib/Wallet/WalletProviderContext.svelte';

    let player = getMusicPlayerContext();
    let wallet = getWalletProviderContext();

    const get_songs = async () => {
        if (!wallet.client) return

        let result = await wallet.client.queryEvents({
            query: {
                MoveEventType: "0xd6eebb11953d91bbc4e5788ab3d1297f9ab4baf52eb2be18449399b08798b6f5::tuno::NFTMinted"
            }
        })

        let songs = result.data
            .map(e => e.parsedJson as SongObject);
        
        songs.forEach(s => player.addNewSong(s));
        return songs
    }
</script>

<div class="px-4 py-12 sm:px-6 lg:px-8">
    <ul role="list" class="divide-y divide-gray-100">
        {#await get_songs() then songs }
            {#if songs}
                {#each songs as song, index}
                    <li>
                        <SongListing {song} {index} />
                    </li>
                {/each}
            {/if}
        {/await}

        <li class="gap-x-6 py-5">
            <AddSongButton />
        </li>
    </ul>
</div>