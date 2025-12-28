macro_rules! deps {
    () => {
        QueryOrigin!();
    };
}

macro_rules! impl_477 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl serde :: Serialize for QueryOrigin { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { self . as_ref () . serialize (serializer) } }
    };
}

impl_477!()