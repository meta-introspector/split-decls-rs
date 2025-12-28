macro_rules! deps {
    () => {
        SerializeFieldMap!();
        SerdeMapVisitor!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Serialize for SerializeFieldMap < '_ , Record < '_ > > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let serializer = serializer . serialize_map (None) ? ; let mut visitor = SerdeMapVisitor :: new (serializer) ; self . 0 . record (& mut visitor) ; visitor . finish () } }
    };
}

impl_7!();