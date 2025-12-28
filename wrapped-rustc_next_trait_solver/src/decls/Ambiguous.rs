macro_rules! Ambiguous {
    () => {
        # [doc = " Marker for bailing with ambiguity."] pub (crate) struct Ambiguous ;
    };
}

Ambiguous!();