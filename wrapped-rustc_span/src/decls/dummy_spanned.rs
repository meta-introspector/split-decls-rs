macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! dummy_spanned {
    () => {
        deps!();
        pub fn dummy_spanned < T > (t : T) -> Spanned < T > { respan (DUMMY_SP , t) }
    };
}

dummy_spanned!();