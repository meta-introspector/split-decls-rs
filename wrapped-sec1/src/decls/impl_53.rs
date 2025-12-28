macro_rules! deps {
    () => {
        EcPrivateKey!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < 'a > Sequence < 'a > for EcPrivateKey < 'a > { }
    };
}

impl_53!()