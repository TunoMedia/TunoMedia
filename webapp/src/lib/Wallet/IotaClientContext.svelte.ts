import { getFullnodeUrl, IotaClient } from "@iota/iota-sdk/client";
import { getContext, setContext } from "svelte";

const iotaClient = new IotaClient({ url: getFullnodeUrl('testnet') });

/*
 * Context definition
 */
const IOTA_CLIENT_KEY = Symbol('IOTA_CLIENT_KEY');

export function setIotaClientContext() {
	return setContext(IOTA_CLIENT_KEY, iotaClient);
}

export function getIotaClientContext() {
	return getContext<ReturnType<typeof setIotaClientContext>>(IOTA_CLIENT_KEY);
}
