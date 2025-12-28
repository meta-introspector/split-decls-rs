macro_rules! deps {
    () => {
        Level!();
        LevelFilter!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl PartialOrd < Level > for LevelFilter { # [inline (always)] fn partial_cmp (& self , other : & Level) -> Option < cmp :: Ordering > { Some ((other . 0 as usize) . cmp (& filter_as_usize (& self . 0))) } # [inline (always)] fn lt (& self , other : & Level) -> bool { (other . 0 as usize) < filter_as_usize (& self . 0) } # [inline (always)] fn le (& self , other : & Level) -> bool { (other . 0 as usize) <= filter_as_usize (& self . 0) } # [inline (always)] fn gt (& self , other : & Level) -> bool { (other . 0 as usize) > filter_as_usize (& self . 0) } # [inline (always)] fn ge (& self , other : & Level) -> bool { (other . 0 as usize) >= filter_as_usize (& self . 0) } }
    };
}

impl_223!()