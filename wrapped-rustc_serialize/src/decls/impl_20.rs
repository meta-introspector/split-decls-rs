macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < D : Decoder , T > Decodable < D > for PhantomData < T > { fn decode (_ : & mut D) -> PhantomData < T > { PhantomData } }
    };
}

impl_20!()