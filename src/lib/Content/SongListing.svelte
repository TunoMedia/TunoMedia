<script lang="ts">
	let { song } = $props();
</script>

<div class="flex min-w-0 gap-x-4 ml-4">
    <div class="min-w-0 flex-auto">
        <p class="text-sm/6 font-semibold text-gray-900">{song.title}</p>
        <p class="mt-1 truncate text-xs/5 text-gray-500">{song.author}</p>
    </div>
</div>
<div class="shrink-0 sm:flex sm:flex-col sm:items-end">
    <!-- <audio id="{song.id}" controls>
        <source src="/{song.id}.mp3" type="audio/mpeg">
    </audio> -->
</div>