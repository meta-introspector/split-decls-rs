macro_rules! deps {
    () => {
        Durability!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl serde :: Serialize for Durability { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serde :: Serialize :: serialize (& (self . 0 as u8) , serializer) } }
    };
}

impl_90!()