macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! Array {
    () => {
        deps!();
        # [doc = " Type representing a TOML array, payload of the `Value::Array` variant"] pub type Array = Vec < Value > ;
    };
}

Array!()