<script lang="ts">
    import Header from '$lib/Header/Header.svelte';
    import Content from '$lib/Content/Content.svelte';
    import Footer from '$lib/Footer/Footer.svelte';
	import TunoSocketDebugger from '$lib/Player/TunoSocketDebugger.svelte';
</script>


<div class="font-inter font-feature-default antialiased">
    {#if import.meta.env.MODE === "development"}
        <div class="row-span-3 bg-green-300 content-center p-4">
            <TunoSocketDebugger />
        </div>
    {/if}

    <div class="h-dvh w-dvw">
        <div class="h-[77%] p-4 bg-gradient-to-b from-sky-300 to-neutral-950">
            <div class="h-[15%]">
                <Header />
            </div>
            <div class="h-[85%] overflow-x-hidden custom-scrollbar overflow-y-scroll">
                <Content />
            </div>
        </div>
        <div class="h-[23%]">
            <Footer />
        </div>
    </div>
</div>
