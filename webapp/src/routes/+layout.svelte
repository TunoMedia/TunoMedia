<script lang="ts">
	import { setIotaClientContext } from '$lib/Wallet/IotaClientContext.svelte';
	import { setMusicPlayerContext } from '$lib/Player/MusicPlayer.svelte';
	
	import '../app.css';

	let { children } = $props();

	setIotaClientContext();
	setMusicPlayerContext();
</script>

{@render children()}
