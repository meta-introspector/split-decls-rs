macro_rules! default_client {
    () => {
        fn default_client () -> Client { let client = Client :: new (32) . expect ("failed to create jobserver") ; client . acquire_raw () . ok () ; client }
    };
}

default_client!()