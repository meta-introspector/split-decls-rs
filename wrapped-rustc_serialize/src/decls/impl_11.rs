macro_rules! deps {
    () => {
        Decoder!();
        Decodable!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < D : Decoder > Decodable < D > for ! { fn decode (_d : & mut D) -> ! { unreachable ! () } }
    };
}

impl_11!()