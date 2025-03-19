<script lang="ts">
    import Header from '$lib/Header/Header.svelte';
    import Content from '$lib/Content/Content.svelte';
    import Footer from '$lib/Footer/Footer.svelte';
	import TunoSocketDebugger from '$lib/Player/TunoSocketDebugger.svelte';
</script>

<div class="grid h-dvh w-dvw grid-rows-[10]">
    {#if import.meta.env.MODE === "development"}
        <div class="row-span-3 bg-green-300 content-center">
            <TunoSocketDebugger />
        </div>
    {/if}

    <div class="row-span-1">
        <Header />
    </div>
    <div class="row-span-8 overflow-x-hidden overflow-y-scroll">
        <Content />
    </div>
    <div class="row-span-1">
        <Footer />
    </div>
</div>




