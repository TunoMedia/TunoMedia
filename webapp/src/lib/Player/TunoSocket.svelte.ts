import { GrpcWebFetchTransport } from '@protobuf-ts/grpcweb-transport'
import { TunoClient } from "$lib/proto/tuno.client";

export class TunoSocket {
    #client: TunoClient | null = null

    constructor(baseUrl: any = "https://tuno.media:4114") {
        this.#client = new TunoClient(
            new GrpcWebFetchTransport({
                baseUrl,
                format: "binary"
            })
        );
    }

    async echo(message: string): Promise<string> {
        if (!this.#client) return Promise.reject("no gRPC client available");

        let { response } = await this.#client.echo({ message });
        return response.message;
    }

    async fetchSong(rawTransaction: string): Promise<Uint8Array> {
        if (!this.#client) return Promise.reject("no gRPC client available");

        let { response } = await this.#client.fetchSong({ rawTransaction });
        return response.data;
    }

    async* streamSong(rawTransaction: string): AsyncGenerator<Uint8Array> {
        if (!this.#client) return Promise.reject("no gRPC client available");

        let streamingCall = this.#client.streamSong({
            rawTransaction,
            blockSize: 4 * 512 * 512
        });

        try {
            for await (let response of streamingCall.responses) {
                yield response.data;
            }
        } finally {
            // Check for error status
            await streamingCall;
        }
    }
}