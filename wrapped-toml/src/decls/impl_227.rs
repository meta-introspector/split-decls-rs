macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl Sealed for str { }
    };
}

impl_227!()