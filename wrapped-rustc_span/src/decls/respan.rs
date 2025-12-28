macro_rules! deps {
    () => {
        Span!();
        Spanned!();
    };
}

macro_rules! respan {
    () => {
        deps!();
        pub fn respan < T > (sp : Span , t : T) -> Spanned < T > { Spanned { node : t , span : sp } }
    };
}

respan!()