macro_rules! deps {
    () => {
        AtomicRevision!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for AtomicRevision { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { serde :: Deserialize :: deserialize (deserializer) . map (| data | Self { data : AtomicUsize :: new (data) , }) } }
    };
}

impl_255!()