macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl serde :: Serialize for SmolStr { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { self . as_str () . serialize (serializer) } }
    };
}

impl_71!();