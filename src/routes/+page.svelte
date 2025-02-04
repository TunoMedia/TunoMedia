<script lang="ts">
    import AddSongButton from '$lib/AddSongButton.svelte';
    import SongListing from '$lib/SongListing.svelte';
    import songs from '../static/songs.json'; 
</script>

<div class="px-4 py-12 sm:px-6 lg:px-8">
    <ul role="list" class="divide-y divide-gray-100">
        {#each songs as song}
            <li class="flex justify-between gap-x-6 py-5 rounded-3xl hover:bg-gray-100">
                <SongListing {song}/>
            </li>
        {/each}

        <li class="gap-x-6 py-5">
            <AddSongButton />
        </li>
    </ul>
</div>
