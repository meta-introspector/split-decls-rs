macro_rules! deps {
    () => {
        SerializeField!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Serialize for SerializeField < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_str (self . 0 . name ()) } }
    };
}

impl_10!()