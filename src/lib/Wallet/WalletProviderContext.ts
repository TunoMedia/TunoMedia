import { 
    createNetworkConfig, 
    IotaClientProvider, 
    useCurrentAccount, 
    WalletProvider 
} from "@iota/dapp-kit";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { getFullnodeUrl } from "@iota/iota-sdk/client";
import { createElement } from "react";
import { createRoot } from "react-dom/client";
import React from "react";

import type { WalletAccount } from "@iota/wallet-standard";

const queryClient = new QueryClient();
const { networkConfig } = createNetworkConfig({
    localnet: { url: getFullnodeUrl('localnet') },
    testnet: { url: getFullnodeUrl('testnet') },
});

let root: any = null;
let initialized = false;

export function initializeWalletProviderContext(rootEl: HTMLElement, ConnectButton: any) {
    if (initialized) return;

    root = createRoot(rootEl);
    root.render(
        createElement(QueryClientProvider, { 
            client: queryClient,
            children: createElement(IotaClientProvider, {
                networks: networkConfig,
                defaultNetwork: 'testnet',
                children: createElement(WalletProvider, {
                    children: [
                      createElement(ConnectButton, { key: 'ConnectButton' }),
                      createElement(AccountBridge, {
                        key: "AccountBridge",
                        onAccountChange: (account) => console.log(`Account changed ${account?.address}`)
                      })
                    ]
                }) 
            })
        })
    );
    
    initialized = true;
  
    return root
}

/*
 * Helper components to access hooks
 */
interface AccountComponentProps {
    onAccountChange: (account: WalletAccount | null) => void;
}

function AccountBridge({ onAccountChange }: AccountComponentProps) {
    const account = useCurrentAccount();
    React.useEffect(() => {
        onAccountChange(account);
    }, [account, onAccountChange]);
    return null;
}