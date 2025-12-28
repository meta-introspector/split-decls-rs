macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl From < & str > for HSTRING { fn from (value : & str) -> Self { unsafe { Self :: from_wide_iter (value . encode_utf16 () , value . len ()) } } }
    };
}

impl_28!();