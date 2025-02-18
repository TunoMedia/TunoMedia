<script lang="ts">
	import { setWalletProviderContext } from '$lib/Wallet/WalletProviderContext.svelte';
	import '../app.css';

	let { children } = $props();

	setWalletProviderContext();
</script>

{@render children()}
