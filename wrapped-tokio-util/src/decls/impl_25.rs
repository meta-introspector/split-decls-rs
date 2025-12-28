macro_rules! deps {
    () => {
        JoinMapKeys!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'a , K , V > ExactSizeIterator for JoinMapKeys < 'a , K , V > { fn len (& self) -> usize { self . iter . len () } }
    };
}

impl_25!();