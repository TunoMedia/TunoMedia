<script lang="ts">
    import { ConnectButton } from "@iota/dapp-kit";
    import "@iota/dapp-kit/dist/index.css";

    import { getWalletProviderContext } from "./WalletProviderContext.svelte";

    let rootEl: HTMLElement;

    $effect(() => {
        let wallet = getWalletProviderContext();
        let root = wallet.initializeWalletProvider(rootEl, ConnectButton);

        return () => root.unmount();
    });
</script>

<div bind:this={rootEl}></div>
