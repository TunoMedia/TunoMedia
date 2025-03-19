<script lang="ts">
    import { TunoSocket } from "./TunoSocket.svelte";

    const client = new TunoSocket("http://localhost:4114");

    let message = $state("");
    let echo_message: Promise<string> = $state(Promise.resolve(""));
    let stream_resp: Promise<Uint8Array> = $state(Promise.resolve(new Uint8Array()));
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
        <h3>stream function</h3>
        <button onclick={() => stream_resp = client.stream("0x42982f243bd4516629d4529a4f4899fb1567f1c0074b0ddf7529c154279ba534")}>stream</button>
        <button onclick={() => stream_resp = client.stream("../../../../../../etc/passwd")}>error</button>
        {#await stream_resp}
            <p>echoing...</p>
        {:then response}
            <p class="bg-blue-400">{response}</p>
        {:catch error}
            <p class="bg-red-400">{error}</p>
        {/await}
    </div>
</div>