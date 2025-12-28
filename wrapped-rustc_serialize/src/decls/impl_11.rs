macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < D : Decoder > Decodable < D > for ! { fn decode (_d : & mut D) -> ! { unreachable ! () } }
    };
}

impl_11!();