macro_rules! deps {
    () => {
        Sha512VarCore!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl HashMarker for Sha512VarCore { }
    };
}

impl_14!()