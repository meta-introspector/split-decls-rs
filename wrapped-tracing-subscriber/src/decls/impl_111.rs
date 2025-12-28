macro_rules! deps {
    () => {
        Layer!();
        NopLayer2!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < S : Subscriber > Layer < S > for NopLayer2 { }
    };
}

impl_111!();