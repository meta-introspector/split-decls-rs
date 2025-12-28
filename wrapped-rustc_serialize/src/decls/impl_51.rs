macro_rules! deps {
    () => {
        Encoder!();
        Encodable!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < S : Encoder , A : Array < Item : Encodable < S > > > Encodable < S > for SmallVec < A > { fn encode (& self , s : & mut S) { self . as_slice () . encode (s) ; } }
    };
}

impl_51!();