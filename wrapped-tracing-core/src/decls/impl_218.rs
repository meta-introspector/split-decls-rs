macro_rules! deps {
    () => {
        Level!();
        LevelFilter!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl PartialOrd < LevelFilter > for Level { # [inline (always)] fn partial_cmp (& self , other : & LevelFilter) -> Option < cmp :: Ordering > { Some (filter_as_usize (& other . 0) . cmp (& (self . 0 as usize))) } # [inline (always)] fn lt (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) < (self . 0 as usize) } # [inline (always)] fn le (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) <= (self . 0 as usize) } # [inline (always)] fn gt (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) > (self . 0 as usize) } # [inline (always)] fn ge (& self , other : & LevelFilter) -> bool { filter_as_usize (& other . 0) >= (self . 0 as usize) } }
    };
}

impl_218!()