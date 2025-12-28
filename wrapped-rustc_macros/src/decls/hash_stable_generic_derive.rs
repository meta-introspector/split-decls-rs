macro_rules! deps {
    () => {
        HashStableMode!();
    };
}

macro_rules! hash_stable_generic_derive {
    () => {
        deps!();
        pub (crate) fn hash_stable_generic_derive (s : synstructure :: Structure < '_ > ,) -> proc_macro2 :: TokenStream { hash_stable_derive_with_mode (s , HashStableMode :: Generic) }
    };
}

hash_stable_generic_derive!();