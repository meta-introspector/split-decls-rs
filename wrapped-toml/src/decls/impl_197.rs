macro_rules! deps {
    () => {
        TraceScope!();
        Style!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl TraceScope { pub (crate) fn new (text : impl core :: fmt :: Display) -> Self { let text = text . to_string () ; let style = anstyle :: Style :: new () ; trace (& format ! ("> {text}") , style) ; Self { text , style , guard : DEBUG_DEPTH . scoped () , } } }
    };
}

impl_197!()