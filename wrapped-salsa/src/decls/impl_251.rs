macro_rules! deps {
    () => {
        Revision!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        # [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for Revision { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { serde :: Deserialize :: deserialize (deserializer) . map (| generation | Self { generation }) } }
    };
}

impl_251!()