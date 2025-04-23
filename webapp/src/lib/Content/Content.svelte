<script lang="ts">
    import SongListing from './SongListing.svelte';
    import { getMusicPlayerContext } from '$lib/Player/MusicPlayer.svelte';
    import { getIotaClientContext } from '$lib/Wallet/IotaClientContext.svelte';

    let player = getMusicPlayerContext();
    let iotaClient = getIotaClientContext();

    const get_songs = async () => {

        let bal = await iotaClient.getBalance({
            owner: "0xe440ce93dbef136eb841f55cc50b198e8cb80ba8de9b379f85ed86827b1a4644",
        });

        console.log(bal);

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

<div class="min-h-full pt-6 px-6 rounded-tr-[50px] bg-white">
    <ul>
        {#await get_songs() then songs }
            {#if songs}
                {#each songs as song, index}
                    <li class="pb-4 h-16">
                        <SongListing {song} {index} />
                    </li>
                    
                {/each}
            {/if}
        {/await}
    </ul>

    <div class="h-[23vh]"></div>
</div>
