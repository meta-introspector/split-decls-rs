macro_rules! deps {
    () => {
        VersionReq!();
        Error!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl Serialize for VersionReq { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_str (self) } }
    };
}

impl_79!()