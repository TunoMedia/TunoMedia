export class WebSocketClient {
    #ws: WebSocket | null = null
    is_open: boolean = $state(false)
    last_response: string | null = $state(null)

    constructor(url: string = "ws://tuno.media:4114") {
        this.#ws = new WebSocket(url)
        this.#ws.onopen = () => this.is_open = true
        this.#ws.onclose = () => this.is_open = false

        this.#ws.onmessage = (event) => this.last_response = event.data
    }

    echo(message: string) {
        this.#ws?.send(JSON.stringify({
            "req": "echo",
            "message": message
        }))
    }


}