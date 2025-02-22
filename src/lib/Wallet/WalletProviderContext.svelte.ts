import { 
    createNetworkConfig, 
    IotaClientProvider, 
    useCurrentAccount, 
    useIotaClient,
    useSignPersonalMessage, 
    WalletProvider, 
} from "@iota/dapp-kit";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { getFullnodeUrl, IotaClient } from "@iota/iota-sdk/client";
import { createElement } from "react";
import { createRoot } from "react-dom/client";
import React from "react";

import type { IotaSignPersonalMessageMethod, WalletAccount } from "@iota/wallet-standard";
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
    client: IotaClient | null = $state(null);
    signPersonalMessage: IotaSignPersonalMessageMethod | undefined = $state();

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
                          createElement(WalletHooksBridge, {
                            key: "WalletHooksBridge",
                            onAccountChange: (account) => this.currentAccount = account,
                            onSignPersonalMessageReady: (signPersonalMessage) => this.signPersonalMessage = signPersonalMessage,
                          }),
                          createElement(RpcHooksBridge, {
                            key: "RpcHooksBridgeBridge",
                            onClientChange: (client) => this.client = client,
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
interface WalletHooksBridgeProps {
    onAccountChange: (account: WalletAccount | null) => void,
    onSignPersonalMessageReady: (signPersonalMessage: any) => void,
}

function WalletHooksBridge({
    onAccountChange,
    onSignPersonalMessageReady,
}: WalletHooksBridgeProps) {
    const { mutate: signPersonalMessage } = useSignPersonalMessage();
    const account = useCurrentAccount();

    React.useEffect(() => {
        onSignPersonalMessageReady(signPersonalMessage);
    }, [signPersonalMessage, onSignPersonalMessageReady]);

    React.useEffect(() => {
        onAccountChange(account);
    }, [account, onAccountChange]);

    return null;
}

interface RpcHooksBridgeProps {
    onClientChange: (client: IotaClient) => void,
} 

function RpcHooksBridge({
    onClientChange,
}: RpcHooksBridgeProps) {
    const client = useIotaClient();

    React.useEffect(() => {
        onClientChange(client);
    }, [client, onClientChange]);

    return null;
}
