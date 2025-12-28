macro_rules! deps {
    () => {
        Config!();
        CustomConfig!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        # [cfg (target_pointer_width = "32")] impl Config for CustomConfig { const INITIAL_PAGE_SIZE : usize = 16 ; const MAX_PAGES : usize = 6 ; const MAX_THREADS : usize = 128 ; const RESERVED_BITS : usize = 12 ; }
    };
}

impl_287!()