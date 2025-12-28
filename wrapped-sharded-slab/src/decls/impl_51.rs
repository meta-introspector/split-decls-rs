macro_rules! deps {
    () => {
        DefaultConfig!();
        Config!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl Config for DefaultConfig { const INITIAL_PAGE_SIZE : usize = 32 ; # [cfg (target_pointer_width = "64")] const MAX_THREADS : usize = 4096 ; # [cfg (target_pointer_width = "32")] const MAX_THREADS : usize = 128 ; const MAX_PAGES : usize = WIDTH / 2 ; }
    };
}

impl_51!();