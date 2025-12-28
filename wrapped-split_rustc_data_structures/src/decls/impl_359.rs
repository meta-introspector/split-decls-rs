macro_rules! deps {
    () => {
        Pu128!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl < S : Encoder > Encodable < S > for Pu128 { # [inline] fn encode (& self , s : & mut S) { { self . 0 } . encode (s) ; } }
    };
}

impl_359!()