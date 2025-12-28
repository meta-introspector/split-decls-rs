macro_rules! deps {
    () => {
        Uuid!();
        Error!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl str :: FromStr for Uuid { type Err = Error ; fn from_str (uuid_str : & str) -> Result < Self , Self :: Err > { Uuid :: parse_str (uuid_str) } }
    };
}

impl_31!()