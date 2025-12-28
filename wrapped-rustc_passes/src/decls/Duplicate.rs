macro_rules! Duplicate {
    () => {
        pub (crate) enum Duplicate { Plain , Crate , CrateDepends , }
    };
}

Duplicate!()