<script lang="ts">
	import { setWalletProviderContext } from '$lib/Wallet/WalletProviderContext.svelte';
	import { setMusicPlayerContext } from '$lib/MusicPlayer.svelte';
	import '../app.css';
	
	import songs from '../static/songs.json';

	let { children } = $props();

	setWalletProviderContext();
	setMusicPlayerContext(songs);
</script>

{@render children()}
