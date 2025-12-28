macro_rules! Builder {
    () => {
        pub struct Builder { intent : ThreadIntent , inner : jod_thread :: Builder , allow_leak : bool , }
    };
}

Builder!();