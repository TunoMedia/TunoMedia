#[test_only]
module tuno::user_tests {
    use std::string;
    use iota::test_scenario;
    use iota::test_utils::assert_eq;
    // use iota::transfer;
    use iota::coin;
    // use iota::object::{Self, ID};
    // use iota::kiosk::{Self, Kiosk, KioskOwnerCap};
    use tuno::tuno::{Self, Song};

    use tuno::constants::{
        get_user,
        get_creator,
        get_distributor1,
        get_streaming_price,
        get_distributor_fee
    };

    use tuno::utils::{
        setup_creator,
        create_test_song,
        list_song_on_kiosk,
        register_distributor
    };

    #[test]
    fun test_full_user_journey() {
        let mut scenario = setup_creator();
        create_test_song(&mut scenario);
        list_song_on_kiosk(&mut scenario);
        
        // Make song ID available for user discovery
        let song_id: ID;
        test_scenario::next_tx(&mut scenario, get_creator());
        {
            let song = test_scenario::take_from_sender<Song>(&scenario);
            song_id = object::id(&song);
            test_scenario::return_to_sender(&scenario, song);
        };
        
        // Transfer song to distributor for testing
        test_scenario::next_tx(&mut scenario, get_creator());
        {
            let song = test_scenario::take_from_sender<Song>(&scenario);
            transfer::public_transfer(song, get_distributor1());
        };
        
        register_distributor(get_distributor1(), b"192.168.1.1:8080", &mut scenario);

        // User Journey - Discover and Pay
        test_scenario::next_tx(&mut scenario, get_user());
        {
            let total_fee = get_streaming_price() + get_distributor_fee();
            let payment = coin::mint_for_testing(total_fee, test_scenario::ctx(&mut scenario));
            
            let mut song = test_scenario::take_from_address<Song>(&scenario, get_distributor1());
            
            assert_eq(object::id(&song), song_id);
            
            let (url, _, fee, _) = tuno::get_distributor_info(&song, get_distributor1());
            assert_eq(url, string::utf8(b"192.168.1.1:8080"));
            assert_eq(fee, get_distributor_fee());
            
            tuno::pay_royalties(&mut song, get_distributor1(), payment);
            
            test_scenario::return_to_address(get_distributor1(), song);
        };
        
        // Verify payments received
        test_scenario::next_tx(&mut scenario, get_creator());
        {
            let song = test_scenario::take_from_address<Song>(&scenario, get_distributor1());
            
            let (_, _, _, _, _, _, creator_balance, _) = tuno::get_song_info(&song);
            assert_eq(creator_balance, get_streaming_price());
            
            let (_, _, _, distributor_balance) = tuno::get_distributor_info(&song, get_distributor1());
            assert_eq(distributor_balance, get_distributor_fee());
            
            test_scenario::return_to_address(get_distributor1(), song);
        };
        
        test_scenario::end(scenario);
    }
}