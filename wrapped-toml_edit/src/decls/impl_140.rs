macro_rules! deps {
    () => {
        Key!();
        TomlError!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        # [cfg (feature = "parse")] impl FromStr for Key { type Err = crate :: TomlError ; # [doc = " Tries to parse a key from a &str,"] # [doc = " if fails, tries as basic quoted key (surrounds with \"\")"] # [doc = " and then literal quoted key (surrounds with '')"] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_parse_simple (s) } }
    };
}

impl_140!();