#[test_only]
module tuno::distributor_tests {
    use iota::coin;
    use std::string;
    use iota::iota::IOTA;
    use iota::test_scenario;
    use iota::test_utils::assert_eq;
    use tuno::tuno::{Self, Song};

    use tuno::constants::{
        get_user,
        get_creator,
        get_distributor1,
        get_distributor2,
        get_streaming_price,
        get_distributor_fee
    };

    use tuno::utils::{
        setup_creator,
        create_test_song,
        place_song_on_kiosk,
        register_distributor
    };

    #[test]
    fun test_distributor_registration_and_update() {
        let mut scenario = setup_creator();
        create_test_song(&mut scenario);
        place_song_on_kiosk(&mut scenario);
        register_distributor(get_distributor1(), b"192.168.1.1:8080", get_distributor_fee(), &mut scenario);
        
        // Check if distributor was registered
        test_scenario::next_tx(&mut scenario, get_distributor1());
        {
            let song = test_scenario::take_shared<Song<IOTA>>(&scenario);
            
            assert_eq(tuno::is_distributor(&song, get_distributor1()), true);
            
            let (url, _, price, balance) = tuno::get_distributor_info(&song, get_distributor1());
            assert_eq(url, string::utf8(b"192.168.1.1:8080"));
            assert_eq(price, get_distributor_fee());
            assert_eq(balance, 0);
            
            test_scenario::return_shared(song);
        };
        
        // Update distributor info
        test_scenario::next_tx(&mut scenario, get_distributor1());
        {
            let mut song = test_scenario::take_shared<Song<IOTA>>(&scenario);
            
            tuno::update_distributor_info(
                &mut song,
                b"10.0.0.1:9090",
                600_000,
                test_scenario::ctx(&mut scenario)
            );
            
            let (url, _, price, _) = tuno::get_distributor_info(&song, get_distributor1());
            assert_eq(url, string::utf8(b"10.0.0.1:9090"));
            assert_eq(price, 600_000);
            
            test_scenario::return_shared(song);
        };
        
        test_scenario::end(scenario);
    }

    #[test]
    fun test_royalty_payments_and_withdrawals() {
        let mut scenario = setup_creator();
        create_test_song(&mut scenario);
        place_song_on_kiosk(&mut scenario);
        register_distributor(get_distributor1(), b"192.168.1.1:8080", get_distributor_fee(), &mut scenario);
        
        // User makes payment for streaming
        test_scenario::next_tx(&mut scenario, get_user());
        {
            let mut song = test_scenario::take_shared<Song<IOTA>>(&scenario);
            let total_price = tuno::get_total_price(&song, get_distributor1());
            
            let payment = coin::mint_for_testing(total_price, test_scenario::ctx(&mut scenario));
            
            tuno::pay_royalties(&mut song, get_distributor1(), payment);
            
            test_scenario::return_shared(song);
        };
        
        // Creator withdraws royalties
        test_scenario::next_tx(&mut scenario, get_creator());
        {
            let mut song = test_scenario::take_shared<Song<IOTA>>(&scenario);
            
            let (_, _, _, _, _, _, creator_balance, _) = tuno::get_song_info(&song);
            assert_eq(creator_balance, get_streaming_price());
            
            tuno::withdraw_creator_royalties(&mut song, test_scenario::ctx(&mut scenario));
            
            let (_, _, _, _, _, _, creator_balance_after, _) = tuno::get_song_info(&song);
            assert_eq(creator_balance_after, 0);
            
            // assert!(test_scenario::has_most_recent_for_sender<iota::coin::Coin<iota::iota::IOTA>>(&scenario), 0);
            
            test_scenario::return_shared(song);
        };
        
        // Distributor withdraws fees
        test_scenario::next_tx(&mut scenario, get_distributor1());
        {
            let mut song = test_scenario::take_shared<Song<IOTA>>(&scenario);
            
            let (_, _, _, distributor_balance) = tuno::get_distributor_info(&song, get_distributor1());
            assert_eq(distributor_balance, get_distributor_fee());
            
            tuno::withdraw_distributor_royalties(&mut song, test_scenario::ctx(&mut scenario));
            
            let (_, _, _, distributor_balance_after) = tuno::get_distributor_info(&song, get_distributor1());
            assert_eq(distributor_balance_after, 0);
            
            // assert!(test_scenario::has_most_recent_for_sender<iota::coin::Coin<iota::iota::IOTA>>(&scenario), 0);
            
            test_scenario::return_shared(song);
        };
        
        test_scenario::end(scenario);
    }

    #[test]
    fun test_multiple_distributors() {
        let mut scenario = setup_creator();
        create_test_song(&mut scenario);
        place_song_on_kiosk(&mut scenario);
        register_distributor(get_distributor1(), b"192.168.1.1:8080", get_distributor_fee(), &mut scenario);
        register_distributor(get_distributor2(), b"192.168.1.2:8082", get_distributor_fee() + 100_000, &mut scenario);
        
        // Check second distributor is available
        test_scenario::next_tx(&mut scenario, get_distributor2());
        {
            let song = test_scenario::take_shared<Song<IOTA>>(&scenario);
            
            let distributors = tuno::get_distributors(&song);
            assert_eq(vector::length(&distributors), 2);
            
            let price1 = tuno::get_total_price(&song, get_distributor1());
            let price2 = tuno::get_total_price(&song, get_distributor2());
            
            assert_eq(price1, get_streaming_price() + get_distributor_fee());
            assert_eq(price2, get_streaming_price() + get_distributor_fee() + 100_000);
            
            test_scenario::return_shared(song);
        };
        
        test_scenario::end(scenario);
    }

    #[test]
    fun test_remove_distributor() {
        let mut scenario = setup_creator();
        create_test_song(&mut scenario);
        place_song_on_kiosk(&mut scenario);        
        register_distributor(get_distributor1(), b"192.168.1.1:8080", get_distributor_fee(), &mut scenario);

        // Remove as distributor
        test_scenario::next_tx(&mut scenario, get_distributor1());
        {
            let mut song = test_scenario::take_shared<Song<IOTA>>(&scenario);
            
            tuno::remove_as_distributor(&mut song, test_scenario::ctx(&mut scenario));
            
            // Check distributor was removed
            assert_eq(tuno::is_distributor(&song, get_distributor1()), false);
            
            // Check distributors list is empty
            let distributors = tuno::get_distributors(&song);
            assert_eq(vector::length(&distributors), 0);
            
            test_scenario::return_shared(song);
        };
        
        test_scenario::end(scenario);
    }
}
