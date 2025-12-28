macro_rules! deps {
    () => {
        Version!();
        Error!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl Serialize for Version { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_str (self) } }
    };
}

impl_78!()