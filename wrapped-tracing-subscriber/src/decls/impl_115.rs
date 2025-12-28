macro_rules! deps {
    () => {
        Layer!();
        StringLayer2!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < S : Subscriber > Layer < S > for StringLayer2 { }
    };
}

impl_115!();