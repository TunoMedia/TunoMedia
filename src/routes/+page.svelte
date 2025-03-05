<script lang="ts">
    import Header from '$lib/Header/Header.svelte';
    import Content from '$lib/Content/Content.svelte';
    import Footer from '$lib/Footer/Footer.svelte';

    import WebSocket from '@tauri-apps/plugin-websocket';

    WebSocket.connect('ws://127.0.0.1:8080').then(ws => {
        ws.addListener(msg => console.log('Received Message:', msg));
        ws.send('Hello World!').then(() => ws.disconnect())
    })

</script>

<div class="grid h-dvh w-dvw grid-rows-[10]">
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




