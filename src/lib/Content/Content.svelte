<script lang="ts">
    import AddSongButton from './AddSongButton.svelte';
    import SongListing from './SongListing.svelte';
    import { getMusicPlayerContext } from '$lib/MusicPlayer.svelte';

    let player = getMusicPlayerContext();
</script>

<div class="px-4 py-12 sm:px-6 lg:px-8">
    <ul role="list" class="divide-y divide-gray-100">
        {#each player.songs as song, index}
            <li>
                <SongListing {song} {index} />
            </li>
        {/each}

        <li class="gap-x-6 py-5">
            <AddSongButton />
        </li>
    </ul>
</div>