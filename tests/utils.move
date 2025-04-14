#[test_only]
module tuno::utils {
    use iota::kiosk::{Kiosk, KioskOwnerCap};
    use tuno::tuno::{Self, CreatorCap, Song};

    use iota::test_scenario::{Self, Scenario};
    
    use tuno::constants::{
        get_creator,
        get_streaming_price,
    };

    public(package) fun setup_creator(): Scenario {
        let mut scenario = test_scenario::begin(get_creator());
        {
            tuno::register_creator(test_scenario::ctx(&mut scenario));
        };

        scenario
    }

    public(package) fun create_test_song(scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, get_creator());
        {
            let creator_cap = test_scenario::take_from_sender<CreatorCap>(scenario);
            tuno::create_song(
                b"Test Song",
                b"Test Artist",
                b"Test Album",
                2025,
                b"Electronic",
                b"http://example.com/cover.jpg",
                get_streaming_price(),
                16 * 1024 * 1024,
                90,
                vector[vector[1, 2, 3], vector[4, 5, 6]],
                &creator_cap,
                test_scenario::ctx(scenario)
            );
            test_scenario::return_to_sender(scenario, creator_cap);
        };
    }

    public(package) fun place_song_on_kiosk(scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, get_creator());
        {
            let mut song = test_scenario::take_from_sender<Song>(scenario);
            let mut kiosk = test_scenario::take_shared<Kiosk>(scenario);
            let cap = test_scenario::take_from_sender<KioskOwnerCap>(scenario);
            
            tuno::make_song_available(&mut song, &mut kiosk, &cap, test_scenario::ctx(scenario));
            
            test_scenario::return_to_sender(scenario, song);
            test_scenario::return_to_sender(scenario, cap);
            test_scenario::return_shared(kiosk);
        };
    }

    public(package) fun register_distributor(
        distributor: address,
        url: vector<u8>,
        price: u64,
        scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, distributor);
        {
            let mut song = test_scenario::take_from_address<Song>(scenario, get_creator());

            tuno::register_as_distributor(
                &mut song,
                url,
                price,
                test_scenario::ctx(scenario)
            );
            
            test_scenario::return_to_address(get_creator(), song);
        };
    }
}
