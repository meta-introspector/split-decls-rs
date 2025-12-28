macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl TryFrom < Value > for serde_json :: Value { type Error = serde_json :: Error ; fn try_from (value : Value) -> Result < Self , Self :: Error > { serde_json :: to_value (value) } }
    };
}

impl_125!()