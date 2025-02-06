<script lang="ts">
    import Header from '$lib/Header/Header.svelte';
    import Content from '$lib/Content/Content.svelte';
    import Footer from '$lib/Footer/Footer.svelte';

    import songs from '../static/songs.json'; 
</script>

<div class="grid h-svh w-dvh grid-rows-[10]">
    <div class="row-span-1">
        <Header />
    </div>
    <div class="row-span-8 overflow-x-hidden overflow-y-scroll">
        <Content {songs} />
    </div>
    <div class="row-span-1">
        <Footer />
    </div>
</div>




