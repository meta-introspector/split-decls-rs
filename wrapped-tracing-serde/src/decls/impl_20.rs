macro_rules! deps {
    () => {
        SerializeEvent!();
        SerializeMetadata!();
        SerdeStructVisitor!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Serialize for SerializeEvent < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut serializer = serializer . serialize_struct ("Event" , 2) ? ; serializer . serialize_field ("metadata" , & SerializeMetadata (self . 0 . metadata ())) ? ; let mut visitor = SerdeStructVisitor { serializer , state : Ok (()) , } ; self . 0 . record (& mut visitor) ; visitor . finish () } }
    };
}

impl_20!()