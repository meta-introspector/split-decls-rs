macro_rules! mask {
    () => {
        # [inline] fn mask (bits : usize) -> usize { usize :: MAX >> ((size_of :: < usize > () * 8) - bits) }
    };
}

mask!()