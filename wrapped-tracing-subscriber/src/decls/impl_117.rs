macro_rules! deps {
    () => {
        StringLayer3!();
        Layer!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < S : Subscriber > Layer < S > for StringLayer3 { }
    };
}

impl_117!()