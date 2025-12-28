macro_rules! deps {
    () => {
        RedactedValue!();
        RedactedValueInner!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl From < String > for RedactedValue { fn from (inner : String) -> Self { if inner . is_empty () { Self { inner : None } } else { Self { inner : Some (RedactedValueInner :: String (inner)) , } } } }
    };
}

impl_229!()