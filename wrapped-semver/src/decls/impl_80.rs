macro_rules! deps {
    () => {
        Error!();
        Comparator!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl Serialize for Comparator { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_str (self) } }
    };
}

impl_80!();