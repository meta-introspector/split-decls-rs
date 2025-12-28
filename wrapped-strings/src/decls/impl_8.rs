macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl TryFrom < & BSTR > for String { type Error = alloc :: string :: FromUtf16Error ; fn try_from (value : & BSTR) -> core :: result :: Result < Self , Self :: Error > { Self :: from_utf16 (value) } }
    };
}

impl_8!();