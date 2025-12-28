macro_rules! deps {
    () => {
        TextSize!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl Serialize for TextSize { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . raw . serialize (serializer) } }
    };
}

impl_43!();