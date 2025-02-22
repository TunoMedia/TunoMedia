module tuno::tuno {
    use std::string;
    use iota::event;

    public struct TunoNFT has key, store {
        id: UID,
        name: string::String,
        author: string::String,
    }

    public struct NFTMinted has copy, drop {
        object_id: ID,
        author: string::String,
        name: string::String,
    }

    public fun mint(
        name: vector<u8>,
        author: vector<u8>,
        ctx: &mut TxContext,
    ): TunoNFT {
        let nft = TunoNFT {
            id: object::new(ctx),
            name: string::utf8(name),
            author: string::utf8(author),
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
        let TunoNFT { id, name: _, author: _ } = nft;
        id.delete()
    }
}
