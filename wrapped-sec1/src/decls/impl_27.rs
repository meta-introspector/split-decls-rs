macro_rules! deps {
    () => {
        Result!();
        Tag!();
        Error!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl TryFrom < u8 > for Tag { type Error = Error ; fn try_from (byte : u8) -> Result < Self > { Self :: from_u8 (byte) } }
    };
}

impl_27!()