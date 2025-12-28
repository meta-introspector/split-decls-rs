macro_rules! deps {
    () => {
        RawString!();
        RawStringInner!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl From < & str > for RawString { # [inline] fn from (s : & str) -> Self { if s . is_empty () { Self (RawStringInner :: Empty) } else { String :: from (s) . into () } } }
    };
}

impl_207!()