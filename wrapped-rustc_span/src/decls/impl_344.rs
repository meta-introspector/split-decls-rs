macro_rules! deps {
    () => {
        ErrorGuaranteed!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl < E : rustc_serialize :: Encoder > Encodable < E > for ErrorGuaranteed { # [inline] fn encode (& self , _e : & mut E) { panic ! ("should never serialize an `ErrorGuaranteed`, as we do not write metadata or \
            incremental caches in case errors occurred") } }
    };
}

impl_344!();