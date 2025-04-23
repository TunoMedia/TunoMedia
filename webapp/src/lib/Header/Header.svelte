<script lang="ts">
	import ConnectButton from "$lib/Wallet/ConnectButton.svelte";
</script>

<div class="flex h-full mx-8 md:mx-16 xl:mx-24 items-center justify-between">
    <div>
      <a href="/">
        <span class="sr-only">Tuno</span>
        <img class="lg:h-20 h-16 w-auto" src="/Logo.png" alt="">
      </a>
    </div>

    <!-- <div class="flex gap-x-12">
      <a href="#" class="text-sm/6 font-semibold text-gray-900">About us</a>
    </div> -->

    <div>
      <ConnectButton />
    </div>
</div>