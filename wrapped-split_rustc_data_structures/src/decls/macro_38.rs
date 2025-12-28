macro_rules! deps {
    () => {
        Fingerprint!();
    };
}

macro_rules! macro_38 {
    () => {
        deps!();
        impl_stable_traits_for_trivial_type ! (Fingerprint) ;
    };
}

macro_38!()