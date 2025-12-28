macro_rules! deps {
    () => {
        Value!();
        ConstValue!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl TryFrom < ConstValue > for serde_json :: Value { type Error = serde_json :: Error ; fn try_from (value : ConstValue) -> Result < Self , Self :: Error > { serde_json :: to_value (value) } }
    };
}

impl_118!()