macro_rules! deps {
    () => {
        NopLayer2!();
        Layer!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < S : Subscriber > Layer < S > for NopLayer2 { }
    };
}

impl_111!()