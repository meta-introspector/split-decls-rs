macro_rules! deps {
    () => {
        DatabaseKeyIndex!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl serde :: Serialize for DatabaseKeyIndex { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serde :: Serialize :: serialize (& (self . key_index , self . ingredient_index) , serializer) } }
    };
}

impl_224!()