<script lang="ts">
    import Header from '$lib/Header/Header.svelte';
    import Content from '$lib/Content/Content.svelte';
    import Footer from '$lib/Footer/Footer.svelte';
</script>


<div class="font-inter font-feature-default antialiased">
    <div class="h-dvh w-dvw relative bg-black">
        <div class="h-full overflow-x-hidden no-scrollbar">
            <div class="h-[12%]">
                <Header />
            </div>
            <div class="h-[88%]">
                <Content />
            </div>
        </div>
        
        <div class="absolute w-full bottom-0 h-[23%]">
            <Footer />
        </div>
    </div>
</div>
