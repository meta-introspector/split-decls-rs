macro_rules! deps {
    () => {
        SerializeMap!();
        Serializer!();
        Error!();
        Table!();
    };
}

macro_rules! impl_375 {
    () => {
        deps!();
        impl ser :: Serialize for Table { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { use serde_core :: ser :: SerializeMap ; let mut map = serializer . serialize_map (Some (self . len ())) ? ; for (k , v) in self { map . serialize_key (k) ? ; map . serialize_value (v) ? ; } map . end () } }
    };
}

impl_375!();