macro_rules! deps {
    () => {
        OwnedEntry!();
        Config!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl < T , C > OwnedEntry < T , C > where C : cfg :: Config , { # [doc = " Returns the key used to access this guard"] pub fn key (& self) -> usize { self . key } # [inline (always)] fn value (& self) -> & T { unsafe { self . value . as_ref () } } }
    };
}

impl_195!();