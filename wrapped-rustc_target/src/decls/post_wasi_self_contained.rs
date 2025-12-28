macro_rules! deps {
    () => {
        CrtObjects!();
    };
}

macro_rules! post_wasi_self_contained {
    () => {
        deps!();
        pub (super) fn post_wasi_self_contained () -> CrtObjects { new (& []) }
    };
}

post_wasi_self_contained!()