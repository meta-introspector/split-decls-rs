macro_rules! deps {
    () => {
        Variant!();
        Style!();
    };
}

macro_rules! effective_style {
    () => {
        deps!();
        fn effective_style (variant : & Variant) -> Style { match variant . style { Style :: Newtype if variant . fields [0] . attrs . skip_serializing () => Style :: Unit , other => other , } }
    };
}

effective_style!();