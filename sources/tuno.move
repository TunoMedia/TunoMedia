module tuno::tuno {
    use std::string;
    use iota::{event, url::{Self, Url}};

    public struct TunoNFT has key, store {
        id: UID,
        name: string::String,
        author: string::String,
        url: Url,
    }

    public struct NFTMinted has copy, drop {
        object_id: ID,
        author: string::String,
        name: string::String,
    }

    public fun mint_to_sender(
        name: vector<u8>,
        author: vector<u8>,
        url: vector<u8>,
        ctx: &mut TxContext,
    ): TunoNFT {
        let nft = TunoNFT {
            id: object::new(ctx),
            name: string::utf8(name),
            author: string::utf8(author),
            url: url::new_unsafe_from_bytes(url),
        };

        event::emit(NFTMinted {
            object_id: object::id(&nft),
            author: nft.author,
            name: nft.name,
        });

        return nft
    }

    public fun transfer(nft: TunoNFT, recipient: address, _: &mut TxContext) {
        transfer::public_transfer(nft, recipient)
    }

    public fun burn(nft: TunoNFT, _: &mut TxContext) {
        let TunoNFT { id, name: _, author: _, url: _ } = nft;
        id.delete()
    }
}
