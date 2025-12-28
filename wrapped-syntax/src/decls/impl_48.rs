macro_rules! deps {
    () => {
        Repr!();
        TokenText!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'a > TokenText < 'a > { pub fn borrowed (text : & 'a str) -> Self { TokenText (Repr :: Borrowed (text)) } pub (crate) fn owned (green : GreenToken) -> Self { TokenText (Repr :: Owned (green)) } pub fn as_str (& self) -> & str { match & self . 0 { & Repr :: Borrowed (it) => it , Repr :: Owned (green) => green . text () , } } }
    };
}

impl_48!();