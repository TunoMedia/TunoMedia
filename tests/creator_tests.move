#[test_only]
module tuno::creator_tests {
    use std::string;
    use iota::test_scenario;
    use iota::test_utils::assert_eq;
    use iota::kiosk::{Self, Kiosk, KioskOwnerCap};
    use tuno::tuno::{Self, CreatorCap, Song};

    use tuno::constants::{
        get_creator,
        get_streaming_price,
    };

    use tuno::utils::{
        setup_creator,
        create_test_song,
        place_song_on_kiosk,
    };
    
    #[test]
    fun test_creator_registration() {
        let mut scenario = setup_creator();
        
        test_scenario::next_tx(&mut scenario, get_creator());
        {
            // Check that creator cap was created and transferred to creator
            assert!(test_scenario::has_most_recent_for_sender<CreatorCap>(&scenario), 0);
            
            // Check that kiosk was created and shared
            assert!(test_scenario::has_most_recent_shared<Kiosk>(), 0);
            
            // Check that kiosk cap was transferred to creator
            assert!(test_scenario::has_most_recent_for_sender<KioskOwnerCap>(&scenario), 0);
        };
        
        test_scenario::end(scenario);
    }

    #[test]
    fun test_song_creation() {
        let mut scenario = setup_creator();
        create_test_song(&mut scenario);
        
        test_scenario::next_tx(&mut scenario, get_creator());
        {
            let song = test_scenario::take_from_sender<Song>(&scenario);
            let (title, artist, album, year, genre, price, _, _) = tuno::get_song_info(&song);
            
            assert_eq(title, string::utf8(b"Test Song"));
            assert_eq(artist, string::utf8(b"Test Artist"));
            assert_eq(album, string::utf8(b"Test Album"));
            assert_eq(year, 2025);
            assert_eq(genre, string::utf8(b"Electronic"));
            assert_eq(price, get_streaming_price());
            assert_eq(tuno::is_available(&song), false);
            
            test_scenario::return_to_sender(&scenario, song);
        };
        
        test_scenario::end(scenario);
    }

    #[test]
    fun test_song_availability() {
        let mut scenario = setup_creator();
        create_test_song(&mut scenario);
        place_song_on_kiosk(&mut scenario);
        
        // Verify song is available
        test_scenario::next_tx(&mut scenario, get_creator());
        {
            let song = test_scenario::take_from_sender<Song>(&scenario);

            assert_eq(tuno::is_available(&song), true);
            
            test_scenario::return_to_sender(&scenario, song);
        };
        
        // Verify song display is in kiosk
        test_scenario::next_tx(&mut scenario, get_creator());
        {
            let kiosk = test_scenario::take_shared<Kiosk>(&scenario);

            assert_eq(kiosk::item_count(&kiosk), 1);

            // TODO: check song display values
            
            test_scenario::return_shared(kiosk);
        };
        
        // Disable the song
        test_scenario::next_tx(&mut scenario, get_creator());
        {
            let mut song = test_scenario::take_from_sender<Song>(&scenario);
            let mut kiosk = test_scenario::take_shared<Kiosk>(&scenario);
            let cap = test_scenario::take_from_sender<KioskOwnerCap>(&scenario);
            
            tuno::make_song_unavailable(&mut song, &mut kiosk, &cap, test_scenario::ctx(&mut scenario));
            
            assert_eq(tuno::is_available(&song), false);
            
            test_scenario::return_to_sender(&scenario, song);
            test_scenario::return_to_sender(&scenario, cap);
            test_scenario::return_shared(kiosk);
        };
        
        // Verify song display is no longer in kiosk
        test_scenario::next_tx(&mut scenario, get_creator());
        {
            let kiosk = test_scenario::take_shared<Kiosk>(&scenario);
            
            assert_eq(kiosk::item_count(&kiosk), 0);
            
            test_scenario::return_shared(kiosk);
        };
        
        test_scenario::end(scenario);
    }
}
