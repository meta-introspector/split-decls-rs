macro_rules! deps {
    () => {
        Durability!();
        DurabilityVal!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for Durability { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { u8 :: deserialize (deserializer) . map (| value | Self (DurabilityVal :: from (value))) } }
    };
}

impl_91!();