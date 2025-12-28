macro_rules! deps {
    () => {
        ErrorGuaranteed!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl < D : rustc_serialize :: Decoder > Decodable < D > for ErrorGuaranteed { # [inline] fn decode (_d : & mut D) -> ErrorGuaranteed { panic ! ("`ErrorGuaranteed` should never have been serialized to metadata or incremental caches") } }
    };
}

impl_345!()