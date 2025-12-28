macro_rules! deps {
    () => {
        LevelFilter!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl Ord for LevelFilter { # [inline (always)] fn cmp (& self , other : & Self) -> cmp :: Ordering { filter_as_usize (& other . 0) . cmp (& filter_as_usize (& self . 0)) } }
    };
}

impl_222!()