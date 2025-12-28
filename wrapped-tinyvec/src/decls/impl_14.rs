macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [cfg (feature = "serde")] # [cfg_attr (docs_rs , doc (cfg (feature = "serde")))] impl < A : Array > Serialize for ArrayVec < A > where A :: Item : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut seq = serializer . serialize_seq (Some (self . len ())) ? ; for element in self . iter () { seq . serialize_element (element) ? ; } seq . end () } }
    };
}

impl_14!()