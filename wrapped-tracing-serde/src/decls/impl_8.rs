macro_rules! deps {
    () => {
        SerializeId!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Serialize for SerializeId < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_tuple_struct ("Id" , 1) ? ; state . serialize_field (& self . 0 . into_u64 ()) ? ; state . end () } }
    };
}

impl_8!()