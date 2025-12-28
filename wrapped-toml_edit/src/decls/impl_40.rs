macro_rules! deps {
    () => {
        TomlError!();
        DocumentMut!();
        Document!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        # [cfg (feature = "parse")] impl FromStr for DocumentMut { type Err = crate :: TomlError ; # [doc = " Parses a document from a &str"] fn from_str (s : & str) -> Result < Self , Self :: Err > { let im = Document :: from_str (s) ? ; Ok (im . into_mut ()) } }
    };
}

impl_40!();