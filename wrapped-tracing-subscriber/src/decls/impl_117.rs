macro_rules! deps {
    () => {
        Layer!();
        StringLayer3!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < S : Subscriber > Layer < S > for StringLayer3 { }
    };
}

impl_117!();