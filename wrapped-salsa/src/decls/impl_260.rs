macro_rules! deps {
    () => {
        OptionalAtomicRevision!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for OptionalAtomicRevision { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { serde :: Deserialize :: deserialize (deserializer) . map (| data | Self { data : AtomicUsize :: new (data) , }) } }
    };
}

impl_260!()