macro_rules! deps {
    () => {
        Config!();
        OwnedRef!();
        Clear!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T , C > OwnedRef < T , C > where T : Clear + Default , C : cfg :: Config , { # [doc = " Returns the key used to access this guard"] pub fn key (& self) -> usize { self . key } # [inline] fn value (& self) -> & T { unsafe { self . inner . value () } } }
    };
}

impl_28!();