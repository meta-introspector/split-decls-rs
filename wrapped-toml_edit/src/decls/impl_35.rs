macro_rules! deps {
    () => {
        TomlError!();
        Document!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        # [cfg (feature = "parse")] impl FromStr for Document < String > { type Err = crate :: TomlError ; # [doc = " Parses a document from a &str"] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: parse (s . to_owned ()) } }
    };
}

impl_35!();