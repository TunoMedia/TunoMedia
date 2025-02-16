<script lang="ts">
    import { ConnectButton } from "@iota/dapp-kit";
    import "@iota/dapp-kit/dist/index.css";

    import { initializeWalletProviderContext } from "./WalletProviderContext";

    let rootEl: HTMLElement;

    $effect(() => {
        let root = initializeWalletProviderContext(rootEl, ConnectButton);

        return () => root.unmount();
    });
</script>

<div bind:this={rootEl}></div>
