<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	import { setIotaClientContext } from '$lib/Wallet/IotaClientContext.svelte';
	import { setMusicPlayerContext } from '$lib/Player/MusicPlayer.svelte';
	
	import '../app.css';

	let { children } = $props();

	invoke("list_distributors");

	setIotaClientContext();
	setMusicPlayerContext();
</script>

{@render children()}
