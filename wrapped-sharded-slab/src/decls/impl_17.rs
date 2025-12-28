macro_rules! deps {
    () => {
        Ref!();
        Clear!();
        Config!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < T , C > Ref < '_ , T , C > where T : Clear + Default , C : cfg :: Config , { # [doc = " Returns the key used to access this guard"] pub fn key (& self) -> usize { self . key } # [inline] fn value (& self) -> & T { unsafe { self . inner . value () } } }
    };
}

impl_17!()