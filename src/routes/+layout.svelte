<script lang="ts">
	import '../app.css';
	let { children } = $props();
</script>

<div class="flex items-center justify-center mt-4">
	<img class="h-36" src="/Logo.png" alt="Tuno Logo">
</div>

{@render children()}
