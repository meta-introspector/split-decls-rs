macro_rules! DeString {
    () => {
        # [doc = " Type representing a TOML string, payload of the `DeValue::String` variant"] pub type DeString < 'i > = Cow < 'i , str > ;
    };
}

DeString!()