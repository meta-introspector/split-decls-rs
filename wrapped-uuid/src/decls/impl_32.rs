macro_rules! deps {
    () => {
        Uuid!();
        Error!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl TryFrom < & '_ str > for Uuid { type Error = Error ; fn try_from (uuid_str : & '_ str) -> Result < Self , Self :: Error > { Uuid :: parse_str (uuid_str) } }
    };
}

impl_32!();