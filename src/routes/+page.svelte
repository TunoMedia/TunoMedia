<script lang="ts">
    import Header from '$lib/Header/Header.svelte';
    import Content from '$lib/Content/Content.svelte';
    import Footer from '$lib/Footer/Footer.svelte';

    import { MusicPlayer } from '$lib/MusicPlayer.svelte';
    import songs from '../static/songs.json';

    const musicPlayer = new MusicPlayer(songs);
</script>

<div class="grid h-dvh w-dvw grid-rows-[10]">
    <div class="row-span-1">
        <Header />
    </div>
    <div class="row-span-8 overflow-x-hidden overflow-y-scroll">
        <Content {musicPlayer} />
    </div>
    <div class="row-span-1">
        <Footer {musicPlayer} />
    </div>
</div>




