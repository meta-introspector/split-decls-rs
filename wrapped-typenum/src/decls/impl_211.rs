macro_rules! deps {
    () => {
        Internal!();
        InternalMarker!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl InternalMarker for Internal { }
    };
}

impl_211!();