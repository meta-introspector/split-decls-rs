macro_rules! deps {
    () => {
        Repr!();
        SmolStr!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl From < SmolStr > for Arc < str > { # [inline (always)] fn from (text : SmolStr) -> Self { match text . 0 { Repr :: Heap (data) => data , _ => text . as_str () . into () , } } }
    };
}

impl_39!()