macro_rules! deps {
    () => {
        RedactedValue!();
        RedactedValueInner!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        # [cfg (feature = "regex")] impl From < regex :: Regex > for RedactedValue { fn from (inner : regex :: Regex) -> Self { Self { inner : Some (RedactedValueInner :: Regex (inner)) , } } }
    };
}

impl_235!();