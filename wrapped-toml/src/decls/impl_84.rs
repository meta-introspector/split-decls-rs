macro_rules! deps {
    () => {
        Error!();
        Value!();
        ValueDeserializer!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        # [cfg (feature = "parse")] impl core :: str :: FromStr for Value { type Err = crate :: de :: Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { use serde_core :: Deserialize as _ ; Self :: deserialize (crate :: de :: ValueDeserializer :: parse (s) ?) } }
    };
}

impl_84!();