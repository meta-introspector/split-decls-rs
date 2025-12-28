macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl PartialOrd for Level { # [inline (always)] fn partial_cmp (& self , other : & Level) -> Option < cmp :: Ordering > { Some (self . cmp (other)) } # [inline (always)] fn lt (& self , other : & Level) -> bool { (other . 0 as usize) < (self . 0 as usize) } # [inline (always)] fn le (& self , other : & Level) -> bool { (other . 0 as usize) <= (self . 0 as usize) } # [inline (always)] fn gt (& self , other : & Level) -> bool { (other . 0 as usize) > (self . 0 as usize) } # [inline (always)] fn ge (& self , other : & Level) -> bool { (other . 0 as usize) >= (self . 0 as usize) } }
    };
}

impl_216!();