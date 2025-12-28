macro_rules! deps {
    () => {
        TryCmd!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl std :: str :: FromStr for TryCmd { type Err = crate :: Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: parse_trycmd (s) } }
    };
}

impl_9!()