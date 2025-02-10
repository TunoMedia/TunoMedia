<script lang="ts">
    import { availableWallets } from '$lib/WalletAdapter.svelte'
	let dialog: any = $state();

    console.log(availableWallets)
</script>

<div>
    <button
        class="flex justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-md font-semibold text-white hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
        onclick={() => dialog.showModal()}
    >
        Connect
    </button>
</div>


<dialog bind:this={dialog} class=" rounded-2xl">
    <div class="absolute top-0 right-0 flex pt-4 pr-2 m:-ml-10 sm:pr-4">
        <button 
            type="button" 
            class="relative rounded-md text-gray-300 hover:text-white focus:ring-2 focus:ring-white focus:outline-hidden"
            onclick={() => dialog.close()}
        >
            <span class="absolute -inset-2.5"></span>
            <span class="sr-only">Close panel</span>
            <svg class="size-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" data-slot="icon">
              <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
            </svg>
        </button>
    </div>

    <div class="flex flex-col justify-center px-6 py-12 lg:px-8">
            {#if availableWallets.length === 0}
                <div class="w-full">
                    <h5>No wallet is available</h5>
                </div>
            {:else}
                <div class="flex flex-col gap-6">
                    {#each availableWallets as wallet }
                        <div class="flex gap-3">
                            <img class="size-8" src={wallet.icon} alt="{wallet.name} icon">
                            <div class="flex items-center justify-center">
                                <span class="text-md font-semibold">{wallet.name}</span>
                            </div>
                        </div>
                    {/each}
                </div>

            {/if}
    </div>
  </dialog>