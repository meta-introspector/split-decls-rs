macro_rules! deps {
    () => {
        Datetime!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [cfg (feature = "serde")] # [cfg (feature = "alloc")] impl serde_core :: ser :: Serialize for Datetime { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde_core :: ser :: Serializer , { use crate :: alloc :: string :: ToString as _ ; use serde_core :: ser :: SerializeStruct ; let mut s = serializer . serialize_struct (NAME , 1) ? ; s . serialize_field (FIELD , & self . to_string ()) ? ; s . end () } }
    };
}

impl_30!()