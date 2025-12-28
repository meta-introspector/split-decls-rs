macro_rules! Level {
    () => {
        pub (crate) enum Level { Fail , Warn , }
    };
}

Level!();