macro_rules! mix {
    () => {
        # [inline (always)] fn mix (w : & mut [u32 ; 80] , t : usize) -> u32 { (w [t - 3] ^ w [t - 8] ^ w [t - 14] ^ w [t - 16]) . rotate_left (1) }
    };
}

mix!()