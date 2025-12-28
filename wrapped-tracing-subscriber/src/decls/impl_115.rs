macro_rules! deps {
    () => {
        StringLayer2!();
        Layer!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < S : Subscriber > Layer < S > for StringLayer2 { }
    };
}

impl_115!()