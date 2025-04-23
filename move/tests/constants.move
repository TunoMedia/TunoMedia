#[test_only]
module tuno::constants {
    const USER: address = @0xA1;
    const CREATOR: address = @0xC1;
    const DISTRIBUTOR1: address = @0xD1;
    const DISTRIBUTOR2: address = @0xD2;

    const STREAMING_PRICE: u64 = 10_000_000;
    const DISTRIBUTOR_FEE: u64 = 500_000;

    public(package) fun get_user(): address {
        USER
    }

    public(package) fun get_creator(): address {
        CREATOR
    }

    public(package) fun get_distributor1(): address {
        DISTRIBUTOR1
    }

    public(package) fun get_distributor2(): address {
        DISTRIBUTOR2
    }

    public(package) fun get_streaming_price(): u64 {
        STREAMING_PRICE
    }

    public(package) fun get_distributor_fee(): u64 {
        DISTRIBUTOR_FEE
    }
}
