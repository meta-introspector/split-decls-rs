macro_rules! deps {
    () => {
        AtomicIterationCount!();
        IterationCount!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for AtomicIterationCount { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { IterationCount :: deserialize (deserializer) . map (Into :: into) } }
    };
}

impl_53!();