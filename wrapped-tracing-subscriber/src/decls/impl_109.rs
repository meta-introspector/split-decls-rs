macro_rules! deps {
    () => {
        NopLayer!();
        Layer!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < S : Subscriber > Layer < S > for NopLayer { }
    };
}

impl_109!()