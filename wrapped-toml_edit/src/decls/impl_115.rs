macro_rules! deps {
    () => {
        Item!();
        TomlError!();
        Value!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        # [cfg (feature = "parse")] impl FromStr for Item { type Err = crate :: TomlError ; # [doc = " Parses a value from a &str"] fn from_str (s : & str) -> Result < Self , Self :: Err > { let value = s . parse :: < Value > () ? ; Ok (Self :: Value (value)) } }
    };
}

impl_115!();