<script lang="ts">
    import { TunoSocket } from "./TunoSocket.svelte";

    const get_all_stream = async (stream_resp: AsyncGenerator<Uint8Array>) => {
        let bytes = 0;
        for await (let response of stream_resp) {
            bytes += response.length;
        }
        return bytes;
    } 

    const client = new TunoSocket("http://localhost:4114");

    let message = $state("");
    let echo_message: Promise<string> = $state(Promise.resolve(""));
    let fetch_resp: Promise<Uint8Array> = $state(Promise.resolve(new Uint8Array()));
    let stream_resp: AsyncGenerator<Uint8Array> = $state((async function* () {})());
</script>

<div>
    <h1 class="text-center">WebSocket Testing Area</h1>
    <div>
        <h3>echo function</h3>
        <form onsubmit={() => echo_message = client.echo(message)}>
            <label for="message">Message</label>
            <input id="message" name="message" type="text" bind:value={message}>
            <input name="submit" type="submit" value="send">
        </form>
        {#await echo_message}
            <p>echoing...</p>
        {:then response}
            <p class="bg-blue-400">{response}</p>
        {:catch error}
            <p class="bg-red-400">{error}</p>
        {/await}
    </div>

    <div>
        <h3>fetchSong function</h3>
        <button onclick={() => fetch_resp = client.fetchSong("0x42982f243bd4516629d4529a4f4899fb1567f1c0074b0ddf7529c154279ba534")}>fect</button>
        <button onclick={() => fetch_resp = client.fetchSong("../../../../../../etc/passwd")}>error</button>
        {#await fetch_resp}
            <p>fetching...</p>
        {:then response}
            <p class="bg-blue-400">received {response.length} bytes</p>
        {:catch error}
            <p class="bg-red-400">{error}</p>
        {/await}
    </div>

    <div>
        <h3>streamSong function</h3>
        <button onclick={() => stream_resp = client.streamSong("0x42982f243bd4516629d4529a4f4899fb1567f1c0074b0ddf7529c154279ba534")}>stream</button>
        <button onclick={() => stream_resp = client.streamSong("../../../../../../etc/passwd")}>error</button>
        {#await get_all_stream(stream_resp)}
            <p>streaming...</p>
        {:then received_bytes}
            <p class="bg-blue-400">received {received_bytes} bytes</p>
        {:catch error}
            <p class="bg-red-400">{error}</p>
        {/await}
    </div>
</div>