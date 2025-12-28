macro_rules! deps {
    () => {
        ExpectedFound!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < T > ExpectedFound < T > { pub fn new (expected : T , found : T) -> Self { ExpectedFound { expected , found } } }
    };
}

impl_33!()