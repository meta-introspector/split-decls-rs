macro_rules! deps {
    () => {
        RedactedValue!();
        RedactedValueInner!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl From < & 'static str > for RedactedValue { fn from (inner : & 'static str) -> Self { if inner . is_empty () { Self { inner : None } } else { Self { inner : Some (RedactedValueInner :: Str (inner)) , } } } }
    };
}

impl_228!()