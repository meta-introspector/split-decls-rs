macro_rules! deps {
    () => {
        SerializeMetadata!();
        SerializeLevel!();
        SerializeFieldSet!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Serialize for SerializeMetadata < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_struct ("Metadata" , 9) ? ; state . serialize_field ("name" , self . 0 . name ()) ? ; state . serialize_field ("target" , self . 0 . target ()) ? ; state . serialize_field ("level" , & SerializeLevel (self . 0 . level ())) ? ; state . serialize_field ("module_path" , & self . 0 . module_path ()) ? ; state . serialize_field ("file" , & self . 0 . file ()) ? ; state . serialize_field ("line" , & self . 0 . line ()) ? ; state . serialize_field ("fields" , & SerializeFieldSet (self . 0 . fields ())) ? ; state . serialize_field ("is_span" , & self . 0 . is_span ()) ? ; state . serialize_field ("is_event" , & self . 0 . is_event ()) ? ; state . end () } }
    };
}

impl_18!()