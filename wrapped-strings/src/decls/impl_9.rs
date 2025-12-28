macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl TryFrom < BSTR > for String { type Error = alloc :: string :: FromUtf16Error ; fn try_from (value : BSTR) -> core :: result :: Result < Self , Self :: Error > { Self :: try_from (& value) } }
    };
}

impl_9!();