macro_rules! deps {
    () => {
        JoinedArgs!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl serde :: ser :: Serialize for JoinedArgs { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { serializer . serialize_str (& self . to_string ()) } }
    };
}

impl_24!()