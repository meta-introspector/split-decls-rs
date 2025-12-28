macro_rules! deps {
    () => {
        SerdeMapVisitor!();
        SerializeFieldMap!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Serialize for SerializeFieldMap < '_ , Attributes < '_ > > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let len = self . 0 . metadata () . fields () . len () ; let serializer = serializer . serialize_map (Some (len)) ? ; let mut visitor = SerdeMapVisitor :: new (serializer) ; self . 0 . record (& mut visitor) ; visitor . finish () } }
    };
}

impl_6!()