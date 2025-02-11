<script lang="ts">
	import ConnectButton from "$lib/ConnectButton.svelte";
</script>

<div class="mx-auto flex mt-2 max-w-7xl items-center justify-between">
    <div class="flex">
      <a href="/" class="-m-1.5 p-1.5">
        <span class="sr-only">Tuno</span>
        <img class="lg:h-20 h-16 w-auto" src="/Logo.png" alt="">
      </a>
    </div>

    <!-- <div class="flex gap-x-12">
      <a href="#" class="text-sm/6 font-semibold text-gray-900">About us</a>
    </div> -->

    <div class="flex flex-1 justify-end">
      <ConnectButton />
    </div>
</div>