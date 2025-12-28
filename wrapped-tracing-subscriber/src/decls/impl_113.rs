macro_rules! deps {
    () => {
        Layer!();
        StringLayer!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < S : Subscriber > Layer < S > for StringLayer { }
    };
}

impl_113!();