<script lang="ts">
  import { getWalletProviderContext } from "$lib/Wallet/WalletProviderContext.svelte";

  let wallet = getWalletProviderContext();
  let dialog: any = $state();
</script>

<div>
    <button
        class="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm/6 font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
        onclick={() => dialog.showModal()}
    >
        Add Song
    </button>
</div>

<dialog bind:this={dialog} class="rounded-2xl">
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

  {#if wallet.currentAccount}
    <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
      <div class="sm:mx-auto sm:w-full sm:max-w-sm">
        <form class="space-y-6" action="#" method="POST">
          <div>
            <label for="title" class="block text-sm/6 font-medium text-gray-900">Title</label>
            <div class="mt-2">
              <input type="text" name="title" id="title" required class="block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6">
            </div>
          </div>
    
          <div>
            <label for="author" class="block text-sm/6 font-medium text-gray-900">Author</label>
            <div class="mt-2">
              <input type="text" name="author" id="author" required class="block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6">
            </div>
          </div>

          <div>
            <label for="file" class="block text-sm/6 font-medium text-gray-900">File</label>
            <div class="mt-2">
              <input type="file" accept="audio/mp3" name="file" id="file" required class="block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6">
            </div>
          </div>
    
          <div>
            <button type="submit" class="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm/6 font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Add</button>
          </div>
        </form>
      </div>
    </div>
  {:else}
    <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
      <div class="sm:mx-auto sm:w-full sm:max-w-sm">
        <span>A wallet is required to add new songs</span>
      </div>
    </div>
  {/if}
</dialog>