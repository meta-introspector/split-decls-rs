macro_rules! deps {
    () => {
        Revision!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl serde :: Serialize for Revision { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serde :: Serialize :: serialize (& self . as_usize () , serializer) } }
    };
}

impl_250!()