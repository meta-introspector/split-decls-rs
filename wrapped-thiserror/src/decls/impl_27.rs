macro_rules! deps {
    () => {
        ThiserrorProvide!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < T > ThiserrorProvide for T where T : Error + ? Sized , { # [inline] fn thiserror_provide < 'a > (& 'a self , request : & mut Request < 'a >) { self . provide (request) ; } }
    };
}

impl_27!();