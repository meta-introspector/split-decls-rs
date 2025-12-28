macro_rules! client {
    () => {
        pub fn client () -> Client { GLOBAL_CLIENT_CHECKED . get () . expect (ACCESS_ERROR) . clone () }
    };
}

client!()