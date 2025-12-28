macro_rules! ResponseT {
    () => {
        trait ResponseT < I : Interner > { fn var_values (& self) -> CanonicalVarValues < I > ; }
    };
}

ResponseT!();