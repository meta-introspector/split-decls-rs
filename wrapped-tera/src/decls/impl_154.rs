macro_rules! deps {
    () => {
        UniqueStrings!();
        Unique!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl UniqueStrings { fn new (case_sensitive : bool) -> UniqueStrings { UniqueStrings { u : Unique :: < String > :: default () , case_sensitive } } }
    };
}

impl_154!();