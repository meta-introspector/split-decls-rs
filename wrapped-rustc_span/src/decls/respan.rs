macro_rules! deps {
    () => {
        Spanned!();
        Span!();
    };
}

macro_rules! respan {
    () => {
        deps!();
        pub fn respan < T > (sp : Span , t : T) -> Spanned < T > { Spanned { node : t , span : sp } }
    };
}

respan!();