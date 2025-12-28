macro_rules! deps {
    () => {
        Layer!();
        NopLayer!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < S : Subscriber > Layer < S > for NopLayer { }
    };
}

impl_109!();