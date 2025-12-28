macro_rules! deps {
    () => {
        EcPrivateKey!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        # [cfg (feature = "pem")] impl PemLabel for EcPrivateKey < '_ > { const PEM_LABEL : & 'static str = "EC PRIVATE KEY" ; }
    };
}

impl_58!();