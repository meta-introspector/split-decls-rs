macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_into_value {
    () => {
        deps!();
        macro_rules ! impl_into_value { ($ variant : ident : $ T : ty) => { impl From <$ T > for Value { # [inline] fn from (val : $ T) -> Value { Value ::$ variant (val . into ()) } } } ; }
    };
}

impl_into_value!()