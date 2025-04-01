#[test_only]
module tuno::utils {
    use iota::kiosk::{Kiosk, KioskOwnerCap};
    use tuno::tuno::{Self, CreatorCap, Song};

    use iota::test_scenario::{Self, Scenario};
    
    use tuno::constants::{
        get_creator,
        get_streaming_price,
        get_distributor_fee
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
                &creator_cap,
                b"Test Song",
                b"Test Artist",
                b"Test Album",
                2025,
                b"Electronic",
                b"http://example.com/cover.jpg",
                get_streaming_price(),
                test_scenario::ctx(scenario)
            );
            test_scenario::return_to_sender(scenario, creator_cap);
        };
    }

    public(package) fun list_song_on_kiosk(scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, get_creator());
        {
            let mut song = test_scenario::take_from_sender<Song>(scenario);
            let mut kiosk = test_scenario::take_shared<Kiosk>(scenario);
            let cap = test_scenario::take_from_sender<KioskOwnerCap>(scenario);
            
            tuno::list_song(&mut song, &mut kiosk, &cap, test_scenario::ctx(scenario));
            
            test_scenario::return_to_sender(scenario, song);
            test_scenario::return_to_sender(scenario, cap);
            test_scenario::return_shared(kiosk);
        };
    }

    public(package) fun register_distributor(scenario: &mut Scenario, distributor: address, port: u16) {
        test_scenario::next_tx(scenario, distributor);
        {
            let mut song = test_scenario::take_from_sender<Song>(scenario);
            
            tuno::register_as_distributor(
                &mut song,
                b"192.168.1.1",
                port,
                get_distributor_fee(),
                test_scenario::ctx(scenario)
            );
            
            test_scenario::return_to_sender(scenario, song);
        };
    }
}