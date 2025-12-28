macro_rules! deps {
    () => {
        Error!();
        Value!();
        ValueDeserializer!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        # [cfg (feature = "parse")] impl std :: str :: FromStr for ValueDeserializer { type Err = Error ; # [doc = " Parses a value from a &str"] fn from_str (s : & str) -> Result < Self , Self :: Err > { let value = s . parse :: < crate :: Value > () . map_err (Error :: from) ? ; Ok (value . into_deserializer ()) } }
    };
}

impl_321!()