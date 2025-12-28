macro_rules! deps {
    () => {
        TinierConfig!();
        Config!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl crate :: Config for TinierConfig { const INITIAL_PAGE_SIZE : usize = 2 ; const MAX_PAGES : usize = 1 ; }
    };
}

impl_252!();