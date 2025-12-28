macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        impl Ord for Level { # [inline (always)] fn cmp (& self , other : & Self) -> cmp :: Ordering { (other . 0 as usize) . cmp (& (self . 0 as usize)) } }
    };
}

impl_217!()