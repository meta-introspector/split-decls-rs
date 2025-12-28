macro_rules! deps {
    () => {
        Layer!();
        Identity!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < S : Subscriber > Layer < S > for Identity { }
    };
}

impl_143!();