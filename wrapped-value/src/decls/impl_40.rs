macro_rules! deps {
    () => {
        ConstValue!();
        Value!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl TryFrom < serde_json :: Value > for ConstValue { type Error = serde_json :: Error ; fn try_from (value : serde_json :: Value) -> Result < Self , Self :: Error > { Self :: deserialize (value) } }
    };
}

impl_40!()