<script lang="ts">
    import { 
        createNetworkConfig, 
        IotaClientProvider, 
        WalletProvider, 
        ConnectButton 
    } from "@iota/dapp-kit";
    import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
    import { getFullnodeUrl } from "@iota/iota-sdk/client";
	import { createElement } from "react";
    import { createRoot } from "react-dom/client";

    import "@iota/dapp-kit/dist/index.css";

    const queryClient = new QueryClient();
    const { networkConfig } = createNetworkConfig({
        localnet: { url: getFullnodeUrl('localnet') },
        testnet: { url: getFullnodeUrl('testnet') },
    });

    let rootEl: HTMLElement

    $effect(() => {
        let root = createRoot(rootEl)
        let provider = createElement(QueryClientProvider, { 
            client: queryClient,
            children: createElement(IotaClientProvider, {
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
