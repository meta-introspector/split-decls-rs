macro_rules! deps {
    () => {
        OptionalAtomicRevision!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl serde :: Serialize for OptionalAtomicRevision { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serde :: Serialize :: serialize (& self . data . load (Ordering :: Relaxed) , serializer) } }
    };
}

impl_259!();