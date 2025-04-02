module tuno::tuno {
    use iota::event;
    use iota::iota::IOTA;
    use iota::coin::{Self, Coin};
    use std::string::{Self, String};
    use iota::vec_map::{Self, VecMap};
    use iota::balance::{Self, Balance};
    use iota::kiosk::{Self, Kiosk, KioskOwnerCap};


    // ======== Error Constants ========

    const ENotDistributor: u64 = 1;
    const ENotOwner: u64 = 2;
    const EInsufficientAmount: u64 = 3;
    const ENotListed: u64 = 4;
    const EAlreadyListed: u64 = 5;
    const ESongNotInKiosk: u64 = 6;
    
    // ======== Events ========

    public struct SongCreated has copy, drop {
        id: ID,
        title: String,
        artist: String,
        streaming_price: u64
    }
    
    public struct SongListed has copy, drop {
        id: ID,
        kiosk_id: ID,
        streaming_price: u64
    }
    
    public struct SongDelisted has copy, drop {
        id: ID,
        kiosk_id: ID
    }
    
    public struct DistributorAdded has copy, drop {
        song_id: ID,
        distributor: address,
        url: String,
        streaming_price: u64
    }
    
    // ======== Objects ========
    
    public struct Song has key, store {
        id: UID,
        title: String,
        artist: String,
        album: String,
        release_year: u64,
        genre: String,
        cover_art_url: String,
        streaming_price: u64,
        owner: address,
        creator_balance: Balance<IOTA>,
        distributors: VecMap<address, Distributor>,
        is_listed: bool
    }
    
    public struct Distributor has store {
        url: String,
        joined_at: u64,
        streaming_price: u64,
        balance: Balance<IOTA>
    }
    
    public struct CreatorCap has key, store {
        id: UID,
        creator: address
    }
    
    public struct SongDisplay has key, store {
        id: UID,
        song_id: ID,
        title: String,
        artist: String,
        genre: String,
        streaming_price: u64,
        cover_art_url: String,
    }
    
    // ======== Public Functions ========
    
    public entry fun register_creator(ctx: &mut TxContext) {
        let creator_cap = CreatorCap {
            id: object::new(ctx),
            creator: tx_context::sender(ctx)
        };
        
        let (kiosk, kiosk_cap) = kiosk::new(ctx);

        transfer::transfer(creator_cap, tx_context::sender(ctx));
        transfer::public_share_object(kiosk);
        transfer::public_transfer(kiosk_cap, tx_context::sender(ctx));
    }
    
    public entry fun create_song(
        _creator_cap: &CreatorCap,
        title: vector<u8>,
        artist: vector<u8>,
        album: vector<u8>,
        release_year: u64,
        genre: vector<u8>,
        cover_art_url: vector<u8>,
        streaming_price: u64,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        let song = Song {
            id: object::new(ctx),
            title: string::utf8(title),
            artist: string::utf8(artist),
            album: string::utf8(album),
            release_year,
            genre: string::utf8(genre),
            cover_art_url: string::utf8(cover_art_url),
            owner: sender,
            streaming_price,
            creator_balance: balance::zero(),
            distributors: vec_map::empty(),
            is_listed: false
        };
        
        event::emit(SongCreated {
            id: object::id(&song),
            title: song.title,
            artist: song.artist,
            streaming_price
        });

        transfer::transfer(song, sender);
    }
    
    public entry fun list_song(
        song: &mut Song,
        kiosk: &mut Kiosk,
        cap: &KioskOwnerCap,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        assert!(sender == song.owner, ENotOwner);
        
        assert!(!song.is_listed, EAlreadyListed);

        let song_display = SongDisplay {
            id: object::new(ctx),
            song_id: object::id(song),
            title: song.title,
            artist: song.artist,
            genre: song.genre,
            streaming_price: song.streaming_price,
            cover_art_url: song.cover_art_url,
        };
        
        kiosk::place(kiosk, cap, song_display);
        
        song.is_listed = true;
        
        event::emit(SongListed {
            id: object::id(song),
            kiosk_id: object::id(kiosk),
            streaming_price: song.streaming_price
        });
    }
    
    public entry fun delist_song(
        song: &mut Song,
        kiosk: &mut Kiosk,
        cap: &KioskOwnerCap,
        song_display_id: ID,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        assert!(sender == song.owner, ENotOwner);
        
        assert!(song.is_listed, ENotListed);
        
        let song_display = kiosk::take<SongDisplay>(kiosk, cap, song_display_id);
        
        assert!(song_display.song_id == object::id(song), ESongNotInKiosk);
        
        let SongDisplay {
            id,
            song_id: _,
            title: _,
            artist: _,
            genre: _,
            streaming_price: _,
            cover_art_url: _,
        } = song_display;
        object::delete(id);
        
        song.is_listed = false;
        
        event::emit(SongDelisted {
            id: object::id(song),
            kiosk_id: object::id(kiosk)
        });
    }
    
    public entry fun register_as_distributor(
        song: &mut Song,
        url: vector<u8>,
        streaming_price: u64,
        ctx: &mut TxContext
    ) {
        assert!(song.is_listed, ENotListed);
        
        let sender = tx_context::sender(ctx);
        
        let distributor = Distributor {
            url: string::utf8(url),
            joined_at: tx_context::epoch(ctx),
            streaming_price,
            balance: balance::zero()
        };

        vec_map::insert(&mut song.distributors, sender, distributor);
        
        event::emit(DistributorAdded {
            song_id: object::id(song),
            distributor: sender,
            url: string::utf8(url),
            streaming_price
        });
    }
    
    public entry fun update_distributor_info(
        song: &mut Song,
        url: vector<u8>,
        streaming_price: u64,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);

        assert!(vec_map::contains(&song.distributors, &sender), ENotDistributor);
        
        let distributor = vec_map::get_mut(&mut song.distributors, &sender);
        distributor.url = string::utf8(url);
        distributor.streaming_price = streaming_price;
        
        event::emit(DistributorAdded {
            song_id: object::id(song),
            distributor: sender,
            url: string::utf8(url),
            streaming_price
        });
    }
    
    public entry fun remove_as_distributor(
        song: &mut Song,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        
        assert!(vec_map::contains(&song.distributors, &sender), ENotDistributor);
        
        let (_, distributor) = vec_map::remove(&mut song.distributors, &sender);
        
        let Distributor {
            url: _,
            joined_at: _,
            streaming_price: _,
            balance
        } = distributor;
        
        balance::destroy_zero(balance);
    }
    
    public entry fun pay_royalties(
        song: &mut Song, 
        distributor_addr: address,
        payment: Coin<IOTA>
    ) {
        assert!(song.is_listed, ENotListed);
        
        assert!(vec_map::contains(&song.distributors, &distributor_addr), ENotDistributor);
        
        let distributor = vec_map::get_mut(&mut song.distributors, &distributor_addr);
        
        let total_price = song.streaming_price + distributor.streaming_price;
        
        let payment_value = coin::value(&payment);
        assert!(payment_value == total_price, EInsufficientAmount);
        
        let mut payment_balance = coin::into_balance(payment);
        
        balance::join(&mut song.creator_balance, balance::split(&mut payment_balance, song.streaming_price));
        balance::join(&mut distributor.balance, balance::split(&mut payment_balance, distributor.streaming_price));
        
        balance::destroy_zero(payment_balance);
    }
    
    public entry fun withdraw_creator_royalties(
        song: &mut Song,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        
        assert!(sender == song.owner, ENotOwner);
        assert!(balance::value(&song.creator_balance) != 0, EInsufficientAmount);
        
        let amount = balance::value(&song.creator_balance);
        let payment = coin::take(&mut song.creator_balance, amount, ctx);
        
        transfer::public_transfer(payment, sender);
    }
    
    public entry fun withdraw_distributor_royalties(
        song: &mut Song,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        
        assert!(vec_map::contains(&song.distributors, &sender), ENotDistributor);
        
        let distributor = vec_map::get_mut(&mut song.distributors, &sender);
        
        assert!(balance::value(&distributor.balance) != 0, EInsufficientAmount);
        
        let amount = balance::value(&distributor.balance);
        let payment = coin::take(&mut distributor.balance, amount, ctx);
        
        transfer::public_transfer(payment, sender);
    }
    
    // ======== View Functions ========
    
    // Get all distributors for a song
    public fun get_distributors(song: &Song): vector<address> {
        vec_map::keys(&song.distributors)
    }
    
    // Get distributor info for a specific distributor
    public fun get_distributor_info(song: &Song, distributor: address): (String, u64, u64, u64) {
        let distributor_info = vec_map::get(&song.distributors, &distributor);
        (
            distributor_info.url,
            distributor_info.joined_at,
            distributor_info.streaming_price,
            balance::value(&distributor_info.balance)
        )
    }
    
    // Get song info including streaming price and creator balance
    public fun get_song_info(song: &Song): (String, String, String, u64, String, u64, u64, bool) {
        (
            song.title,
            song.artist,
            song.album,
            song.release_year,
            song.genre,
            song.streaming_price,
            balance::value(&song.creator_balance),
            song.is_listed
        )
    }
    
    // Get total streaming price (creator + distributor)
    public fun get_total_price(song: &Song, distributor: address): u64 {
        assert!(vec_map::contains(&song.distributors, &distributor), ENotDistributor);
        let distributor_info = vec_map::get(&song.distributors, &distributor);
        song.streaming_price + distributor_info.streaming_price
    }
    
    // Check if an address is a distributor for a song
    public fun is_distributor(song: &Song, distributor: address): bool {
        vec_map::contains(&song.distributors, &distributor)
    }
    
    // Check if a song is currently listed
    public fun is_listed(song: &Song): bool {
        song.is_listed
    }
}
