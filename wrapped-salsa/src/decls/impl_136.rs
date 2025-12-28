macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for Id { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { serde :: Deserialize :: deserialize (deserializer) . map (Self :: from_bits) } }
    };
}

impl_136!();