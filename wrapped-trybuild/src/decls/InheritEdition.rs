macro_rules! deps {
    () => {
        True!();
    };
}

macro_rules! InheritEdition {
    () => {
        deps!();
        # [derive (Deserialize)] # [serde (deny_unknown_fields)] pub (crate) struct InheritEdition { # [allow (dead_code)] pub workspace : True , }
    };
}

InheritEdition!()