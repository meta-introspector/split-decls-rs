macro_rules! deps {
    () => {
        Table!();
        Error!();
    };
}

macro_rules! impl_374 {
    () => {
        deps!();
        # [cfg (feature = "parse")] impl core :: str :: FromStr for Table { type Err = crate :: de :: Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { crate :: from_str (s) } }
    };
}

impl_374!()