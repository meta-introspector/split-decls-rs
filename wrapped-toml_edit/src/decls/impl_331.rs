macro_rules! deps {
    () => {
        Deserializer!();
        Error!();
        Document!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        # [cfg (feature = "parse")] impl std :: str :: FromStr for Deserializer { type Err = Error ; # [doc = " Parses a document from a &str"] fn from_str (s : & str) -> Result < Self , Self :: Err > { let doc : crate :: Document < _ > = s . parse () . map_err (Error :: from) ? ; Ok (Self :: from (doc)) } }
    };
}

impl_331!();