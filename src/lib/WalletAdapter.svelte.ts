import { getWallets } from '@mysten/wallet-standard';

export const availableWallets = getWallets().get();

