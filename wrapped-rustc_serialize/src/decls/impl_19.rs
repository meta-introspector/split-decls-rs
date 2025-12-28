macro_rules! deps {
    () => {
        Encoder!();
        Encodable!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < S : Encoder , T > Encodable < S > for PhantomData < T > { fn encode (& self , _s : & mut S) { } }
    };
}

impl_19!();