macro_rules! deps {
    () => {
        RedactedValue!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl From < Cow < 'static , str > > for RedactedValue { fn from (inner : Cow < 'static , str >) -> Self { match inner { Cow :: Borrowed (s) => s . into () , Cow :: Owned (s) => s . into () , } } }
    };
}

impl_231!()