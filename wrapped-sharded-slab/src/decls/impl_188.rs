macro_rules! deps {
    () => {
        Config!();
        Entry!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < T , C : cfg :: Config > Entry < '_ , T , C > { # [doc = " Returns the key used to access the guard."] pub fn key (& self) -> usize { self . key } # [inline (always)] fn value (& self) -> & T { unsafe { self . value . as_ref () } } }
    };
}

impl_188!()