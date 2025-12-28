macro_rules! deps {
    () => {
        Sha256VarCore!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl HashMarker for Sha256VarCore { }
    };
}

impl_2!();