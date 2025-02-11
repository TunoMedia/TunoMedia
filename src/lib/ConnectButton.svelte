<script lang="ts">
    import { 
        createNetworkConfig, 
        SuiClientProvider, 
        WalletProvider, 
        ConnectButton 
    } from "@mysten/dapp-kit";
    import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
    import { getFullnodeUrl } from "@mysten/sui/client";
	import { createElement } from "react";
    import { createRoot } from "react-dom/client";

    import "@mysten/dapp-kit/dist/index.css";

    const queryClient = new QueryClient();
    const { networkConfig } = createNetworkConfig({
        testnet: { url: getFullnodeUrl('testnet') },
        mainnet: { url: getFullnodeUrl('mainnet') },
    });

    let rootEl: HTMLElement

    $effect(() => {
        let root = createRoot(rootEl)
        let provider = createElement(QueryClientProvider, { 
            client: queryClient,
            children: createElement(SuiClientProvider, {
                networks: networkConfig,
                defaultNetwork: 'testnet',
                children: createElement(WalletProvider, {
                    children: createElement(ConnectButton)
                }) 
            })
        })

        root.render(provider)
        return () => root.unmount()
    })
</script>

<div bind:this={rootEl}></div>
