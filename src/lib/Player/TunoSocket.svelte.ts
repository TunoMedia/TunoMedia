type ClientState = {
    is_open: boolean,
    is_busy: boolean,
}

export class TunoSocket {
    #ws: WebSocket | null = null
    state: ClientState = $state({
        is_open: false,
        is_busy: false
    })
    last_response: string | null = $state(null)

    constructor(url: string = "wss://tuno.media:4114") {
        this.#ws = new WebSocket(url)
        this.#ws.onopen = this.#onopen;
        this.#ws.onclose = this.#onclose;
    }

    #onopen() {
        this.state = {
            is_open: true,
            is_busy: false
        }
    }

    #onclose() {
        this.state = {
            is_open: false,
            is_busy: false
        }
    }

    #free_state() {
        if (this.#ws) this.#ws.onmessage = null;
        this.state.is_busy = false;
    }

    echo = (message: string): Promise<string> => new Promise((resolve, reject) => {
        if (!this.#ws) return reject("no websocket available");
        if (this.#ws.readyState !== WebSocket.OPEN) return reject("websocket is not open");
        if (this.state.is_busy) return reject("websocket is busy");

        this.state.is_busy = true;
        this.#ws.onmessage = (event) => {
            try {
                const data = JSON.parse(event.data);
                if (data.status !== "success") return reject(`Error: ${data.error}`);
                return resolve(data.message);
            } catch (error) {
                return reject("Failed to parse response: " + error);
            } finally {
                this.#free_state();
            }
        }

        try {
            this.#ws.send(JSON.stringify({
                "req": "echo",
                "message": message
            }));
        } catch (error) {
            this.#free_state();
            reject("Failed to send message: " + error);
        }
    });

    stream = (object_id: string): Promise<Blob> => new Promise((resolve, reject) => {
        if (!this.#ws) return reject("no websocket available");
        if (this.#ws.readyState !== WebSocket.OPEN) return reject("websocket is not open");
        if (this.state.is_busy) return reject("websocket is busy");

        this.state.is_busy = true;
        this.#ws.onmessage = (event) => {
            try {
                if (event.data instanceof Blob) return resolve(event.data)
                return reject(`Error: ${JSON.parse(event.data).error}`);
            } catch (error) {
                return reject("Failed to parse response: " + error);
            } finally {
                this.#free_state();
            }
        }

        try {
            this.#ws.send(JSON.stringify({
                "req": "stream",
                "object_id": object_id
            }));
        } catch (error) {
            this.#free_state();
            reject("Failed to send message: " + error);
        }
    });
}