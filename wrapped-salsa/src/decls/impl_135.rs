macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl serde :: Serialize for Id { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serde :: Serialize :: serialize (& self . as_bits () , serializer) } }
    };
}

impl_135!();