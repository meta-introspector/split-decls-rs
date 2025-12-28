macro_rules! deps {
    () => {
        RawString!();
        RawStringInner!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl From < Box < str > > for RawString { # [inline] fn from (s : Box < str >) -> Self { if s . is_empty () { Self (RawStringInner :: Empty) } else { String :: from (s) . into () } } }
    };
}

impl_210!()