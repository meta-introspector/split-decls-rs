macro_rules! deps {
    () => {
        Unique!();
    };
}

macro_rules! UniqueStrings {
    () => {
        deps!();
        struct UniqueStrings { u : Unique < String > , case_sensitive : bool , }
    };
}

UniqueStrings!()