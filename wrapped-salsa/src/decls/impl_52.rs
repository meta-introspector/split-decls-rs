macro_rules! deps {
    () => {
        AtomicIterationCount!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl serde :: Serialize for AtomicIterationCount { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { self . load () . serialize (serializer) } }
    };
}

impl_52!();