macro_rules! deps {
    () => {
        SerializeAttributes!();
        SerdeStructVisitor!();
        SerializeMetadata!();
        SerializeId!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Serialize for SerializeAttributes < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut serializer = serializer . serialize_struct ("Attributes" , 3) ? ; serializer . serialize_field ("metadata" , & SerializeMetadata (self . 0 . metadata ())) ? ; serializer . serialize_field ("parent" , & self . 0 . parent () . map (SerializeId)) ? ; serializer . serialize_field ("is_root" , & self . 0 . is_root ()) ? ; let mut visitor = SerdeStructVisitor { serializer , state : Ok (()) , } ; self . 0 . record (& mut visitor) ; visitor . finish () } }
    };
}

impl_22!();