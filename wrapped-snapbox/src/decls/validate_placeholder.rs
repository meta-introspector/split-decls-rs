macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! validate_placeholder {
    () => {
        deps!();
        fn validate_placeholder (placeholder : & 'static str) -> crate :: assert :: Result < & 'static str > { if ! placeholder . starts_with ('[') || ! placeholder . ends_with (']') { return Err (format ! ("Key `{placeholder}` is not enclosed in []") . into ()) ; } if placeholder [1 .. (placeholder . len () - 1)] . find (| c : char | ! c . is_ascii_uppercase () && c != '_') . is_some () { return Err (format ! ("Key `{placeholder}` can only be A-Z but ") . into ()) ; } Ok (placeholder) }
    };
}

validate_placeholder!()