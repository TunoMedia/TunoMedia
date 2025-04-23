module tuno::tuno {
    use iota::event;
    use iota::coin::{Self, Coin};
    use std::string::{Self, String};
    use iota::vec_map::{Self, VecMap};
    use iota::balance::{Self, Balance};
    use iota::kiosk::{Self, Kiosk, KioskOwnerCap};


    // ======== Error Constants ========

    const ENotDistributor: u64 = 1;
    const ENotOwner: u64 = 2;
    const EInsufficientAmount: u64 = 3;
    const ENotAvailable: u64 = 4;
    const EAlreadyAvailable: u64 = 5;
    const ESongNotInKiosk: u64 = 6;
    
    // ======== Events ========

    public struct SongCreated<phantom T: drop> has copy, drop {
        id: ID,
        title: String,
        artist: String,
        streaming_price: u64
    }
    
    public struct SongBecameAvailable<phantom T: drop> has copy, drop {
        id: ID,
        kiosk_id: ID,
        streaming_price: u64
    }
    
    public struct SongBecameUnavailable has copy, drop {
        id: ID,
        kiosk_id: ID
    }
    
    public struct DistributorAdded<phantom T: drop> has copy, drop {
        song_id: ID,
        distributor: address,
        url: String,
        streaming_price: u64
    }

    public struct DistributorRemoved has copy, drop {
        song_id: ID,
        distributor: address
    }
    
    // ======== Objects ========
    
    public struct Song<phantom T: drop> has key, store {
        id: UID,
        title: String,
        artist: String,
        album: String,
        release_year: u64,
        genre: String,
        cover_art_url: String,
        streaming_price: u64,
        owner: address,
        length: u64,
        duration: u64,
        signature: vector<vector<u8>>,
        creator_balance: Balance<T>,
        distributors: VecMap<address, Distributor<T>>,
        display_id: Option<ID>
    }
    
    public struct Distributor<phantom T: drop> has store {
        url: String,
        joined_at: u64,
        streaming_price: u64,
        balance: Balance<T>
    }
    
    public struct CreatorCap has key, store {
        id: UID,
        creator: address
    }
    
    public struct SongDisplay<phantom T: drop> has key, store {
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
    
    public entry fun create_song<T: drop>(
        title: vector<u8>,
        artist: vector<u8>,
        album: vector<u8>,
        release_year: u64,
        genre: vector<u8>,
        cover_art_url: vector<u8>,
        streaming_price: u64,
        length: u64,
        duration: u64,
        signature: vector<vector<u8>>,
        _creator_cap: &CreatorCap,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        let song = Song<T> {
            id: object::new(ctx),
            title: string::utf8(title),
            artist: string::utf8(artist),
            album: string::utf8(album),
            release_year,
            genre: string::utf8(genre),
            cover_art_url: string::utf8(cover_art_url),
            owner: sender,
            length,
            duration,
            signature,
            streaming_price,
            creator_balance: balance::zero(),
            distributors: vec_map::empty(),
            display_id: option::none()
        };
        
        event::emit(SongCreated<T> {
            id: object::id(&song),
            title: song.title,
            artist: song.artist,
            streaming_price
        });

        transfer::share_object(song);
    }
    
    public entry fun make_song_available<T: drop>(
        song: &mut Song<T>,
        kiosk: &mut Kiosk,
        cap: &KioskOwnerCap,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        assert!(sender == song.owner, ENotOwner);
        
        assert!(!is_available(song), EAlreadyAvailable);

        let song_display = SongDisplay<T> {
            id: object::new(ctx),
            song_id: object::id(song),
            title: song.title,
            artist: song.artist,
            genre: song.genre,
            streaming_price: song.streaming_price,
            cover_art_url: song.cover_art_url,
        };

        song.display_id = option::some(object::id(&song_display));
        
        kiosk::place(kiosk, cap, song_display);
                
        event::emit(SongBecameAvailable<T> {
            id: object::id(song),
            kiosk_id: object::id(kiosk),
            streaming_price: song.streaming_price
        });
    }
    
    public entry fun make_song_unavailable<T: drop>(
        song: &mut Song<T>,
        kiosk: &mut Kiosk,
        cap: &KioskOwnerCap,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        assert!(sender == song.owner, ENotOwner);
        
        assert!(is_available(song), ENotAvailable);
        
        let song_display = kiosk::take<SongDisplay<T>>(kiosk, cap, song.display_id.extract());
        
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
        
        song.display_id = option::none();
        
        event::emit(SongBecameUnavailable {
            id: object::id(song),
            kiosk_id: object::id(kiosk)
        });
    }
    
    public entry fun register_as_distributor<T: drop>(
        song: &mut Song<T>,
        url: vector<u8>,
        streaming_price: u64,
        ctx: &mut TxContext
    ) {
        assert!(is_available(song), ENotAvailable);
        
        let sender = tx_context::sender(ctx);
        
        let distributor = Distributor {
            url: string::utf8(url),
            joined_at: tx_context::epoch(ctx),
            streaming_price,
            balance: balance::zero()
        };

        vec_map::insert(&mut song.distributors, sender, distributor);
        
        event::emit(DistributorAdded<T> {
            song_id: object::id(song),
            distributor: sender,
            url: string::utf8(url),
            streaming_price
        });
    }
    
    public entry fun update_distributor_info<T: drop>(
        song: &mut Song<T>,
        url: vector<u8>,
        streaming_price: u64,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);

        assert!(vec_map::contains(&song.distributors, &sender), ENotDistributor);
        
        let distributor = vec_map::get_mut(&mut song.distributors, &sender);
        distributor.url = string::utf8(url);
        distributor.streaming_price = streaming_price;
        
        event::emit(DistributorAdded<T> {
            song_id: object::id(song),
            distributor: sender,
            url: string::utf8(url),
            streaming_price
        });
    }
    
    public entry fun remove_as_distributor<T: drop>(
        song: &mut Song<T>,
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

        event::emit(DistributorRemoved {
            song_id: object::id(song),
            distributor: sender
        });
    }
    
    public entry fun pay_royalties<T: drop>(
        song: &mut Song<T>, 
        distributor_addr: address,
        payment: Coin<T>
    ) {
        assert!(is_available(song), ENotAvailable);
        
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
    
    public entry fun withdraw_creator_royalties<T: drop>(
        song: &mut Song<T>,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        
        assert!(sender == song.owner, ENotOwner);
        assert!(balance::value(&song.creator_balance) != 0, EInsufficientAmount);
        
        let amount = balance::value(&song.creator_balance);
        let payment = coin::take(&mut song.creator_balance, amount, ctx);
        
        transfer::public_transfer(payment, sender);
    }
    
    public entry fun withdraw_distributor_royalties<T: drop>(
        song: &mut Song<T>,
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
    public fun get_distributors<T: drop>(song: &Song<T>): vector<address> {
        vec_map::keys(&song.distributors)
    }
    
    // Get distributor info for a specific distributor
    public fun get_distributor_info<T: drop>(song: &Song<T>, distributor: address): (String, u64, u64, u64) {
        let distributor_info = vec_map::get(&song.distributors, &distributor);
        (
            distributor_info.url,
            distributor_info.joined_at,
            distributor_info.streaming_price,
            balance::value(&distributor_info.balance)
        )
    }
    
    // Get song info including streaming price
    public fun get_song_info<T: drop>(song: &Song<T>): (String, String, String, u64, String, u64, u64, Option<ID>) {
        (
            song.title,
            song.artist,
            song.album,
            song.release_year,
            song.genre,
            song.streaming_price,
            balance::value(&song.creator_balance),
            song.display_id
        )
    }
    
    // Get total streaming price (creator + distributor)
    public fun get_total_price<T: drop>(song: &Song<T>, distributor: address): u64 {
        assert!(vec_map::contains(&song.distributors, &distributor), ENotDistributor);
        let distributor_info = vec_map::get(&song.distributors, &distributor);
        song.streaming_price + distributor_info.streaming_price
    }
    
    // Check if an address is a distributor for a song
    public fun is_distributor<T: drop>(song: &Song<T>, distributor: address): bool {
        vec_map::contains(&song.distributors, &distributor)
    }
    
    // Check if a song is currently available
    public fun is_available<T: drop>(song: &Song<T>): bool {
        song.display_id.is_some()
    }
}
