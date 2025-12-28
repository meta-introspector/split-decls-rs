macro_rules! deps {
    () => {
        Config!();
        CustomConfig!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        # [cfg (target_pointer_width = "64")] impl Config for CustomConfig { const INITIAL_PAGE_SIZE : usize = 32 ; const MAX_PAGES : usize = 15 ; const MAX_THREADS : usize = 256 ; const RESERVED_BITS : usize = 24 ; }
    };
}

impl_286!()