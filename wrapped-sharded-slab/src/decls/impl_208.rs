macro_rules! deps {
    () => {
        Config!();
        CustomConfig!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        # [cfg (not (target_pointer_width = "64"))] impl Config for CustomConfig { const INITIAL_PAGE_SIZE : usize = 16 ; const MAX_PAGES : usize = 6 ; const MAX_THREADS : usize = 128 ; const RESERVED_BITS : usize = 12 ; }
    };
}

impl_208!()