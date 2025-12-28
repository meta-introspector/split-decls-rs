macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl TryFrom < & HSTRING > for String { type Error = alloc :: string :: FromUtf16Error ; fn try_from (hstring : & HSTRING) -> core :: result :: Result < Self , Self :: Error > { Self :: from_utf16 (hstring) } }
    };
}

impl_64!()