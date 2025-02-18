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
import { getContext, setContext } from "svelte";

const queryClient = new QueryClient();
const { networkConfig } = createNetworkConfig({
    localnet: { url: getFullnodeUrl('localnet') },
    testnet: { url: getFullnodeUrl('testnet') },
});

export class IotaWalletProvider {
    #root: any = null;
    #initialized = false;

    currentAccount: WalletAccount | null = $state(null);

    initializeWalletProvider(rootEl: HTMLElement, ConnectButton: any) {
        if (this.#initialized) return;
    
        this.#root = createRoot(rootEl);
        this.#root.render(
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
                            onAccountChange: (account) => this.currentAccount = account
                          })
                        ]
                    }) 
                })
            })
        );
        
        this.#initialized = true;
      
        return this.#root
    }
}

/*
 * Context definition
 */
const WALLET_PROVIDER_KEY = Symbol('WALLET_PROVIDER_KEY');

export function setWalletProviderContext() {
	return setContext(WALLET_PROVIDER_KEY, new IotaWalletProvider());
}

export function getWalletProviderContext() {
	return getContext<ReturnType<typeof setWalletProviderContext>>(WALLET_PROVIDER_KEY);
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