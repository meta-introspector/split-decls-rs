macro_rules! deps {
    () => {
        Exfiltrator!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < E : sealed :: Exfiltrator > Exfiltrator for E { }
    };
}

impl_40!()