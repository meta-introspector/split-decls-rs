macro_rules! deps {
    () => {
        AtomicRevision!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl serde :: Serialize for AtomicRevision { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serde :: Serialize :: serialize (& self . data . load (Ordering :: Relaxed) , serializer) } }
    };
}

impl_254!()