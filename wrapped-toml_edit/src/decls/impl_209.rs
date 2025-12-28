macro_rules! deps {
    () => {
        RawStringInner!();
        RawString!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl From < & String > for RawString { # [inline] fn from (s : & String) -> Self { if s . is_empty () { Self (RawStringInner :: Empty) } else { String :: from (s) . into () } } }
    };
}

impl_209!()