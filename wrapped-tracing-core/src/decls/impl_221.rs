macro_rules! deps {
    () => {
        LevelFilter!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl PartialOrd for LevelFilter { # [inline (always)] fn partial_cmp (& self , other : & LevelFilter) -> Option < cmp :: Ordering > { Some (self . cmp (other)) } # [inline (always)] fn lt (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) < filter_as_usize (& self . 0) } # [inline (always)] fn le (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) <= filter_as_usize (& self . 0) } # [inline (always)] fn gt (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) > filter_as_usize (& self . 0) } # [inline (always)] fn ge (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) >= filter_as_usize (& self . 0) } }
    };
}

impl_221!()