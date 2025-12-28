macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl TryFrom < serde_json :: Value > for Value { type Error = serde_json :: Error ; fn try_from (value : serde_json :: Value) -> Result < Self , Self :: Error > { Self :: deserialize (value) } }
    };
}

impl_124!()