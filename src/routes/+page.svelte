<script lang="ts">
    import songs from '../static/songs.json'; 
</script>

<div class="px-4 py-12 sm:px-6 lg:px-8">
    <ul role="list" class="divide-y divide-gray-100">
        {#each songs as song}
            <li class="flex justify-between gap-x-6 py-5 rounded-3xl hover:bg-gray-100">
                <div class="flex min-w-0 gap-x-4 ml-4">
                    <div class="min-w-0 flex-auto">
                        <p class="text-sm/6 font-semibold text-gray-900">{song.title}</p>
                        <p class="mt-1 truncate text-xs/5 text-gray-500">{song.author}</p>
                    </div>
                </div>
                <div class="shrink-0 sm:flex sm:flex-col sm:items-end">
                    <audio id="{song.id}" controls>
                        <source src="/{song.id}.mp3" type="audio/mpeg">
                    </audio>
                </div>
            </li>
        {/each}
    </ul>
</div>
