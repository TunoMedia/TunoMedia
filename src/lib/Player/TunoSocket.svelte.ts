import { GrpcWebFetchTransport } from '@protobuf-ts/grpcweb-transport'
import { TunoClient } from "$lib/proto/tuno.client";

export class TunoSocket {
    #client: TunoClient | null = null

    constructor(baseUrl: string = "https://tuno.media:4114") {
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

    async stream(objectId: string): Promise<Uint8Array> {
        if (!this.#client) return Promise.reject("no gRPC client available");

        let { response } = await this.#client.stream({ objectId });
        return response.data;
    }
}