macro_rules! deps {
    () => {
        Encoder!();
        Encodable!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < S : Encoder , T : Encodable < S > > Encodable < S > for RefCell < T > { fn encode (& self , s : & mut S) { self . borrow () . encode (s) ; } }
    };
}

impl_45!()