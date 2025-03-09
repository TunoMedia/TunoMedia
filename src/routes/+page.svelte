<script lang="ts">
    import Header from '$lib/Header/Header.svelte';
    import Content from '$lib/Content/Content.svelte';
    import Footer from '$lib/Footer/Footer.svelte';
	import { WebSocketClient } from '$lib/Player/WebsocketClient.svelte';

    const client = new WebSocketClient();

    let message = $state("")
</script>

<div class="grid h-dvh w-dvw grid-rows-[10]">
    <div class="row-span-3 bg-green-300 content-center">
        <h1 class="text-center">Websocket Testing Area</h1>
        <span>echo function</span>
        <form onsubmit={() => client.echo(message)}>
            <label for="message">Message</label>
            <input id="message" name="message" type="text" bind:value={message}>
            <input name="submit" type="submit" value="send">
        </form>
        <span>Response:</span>
        <span>{client.last_response}</span>
    </div>

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




