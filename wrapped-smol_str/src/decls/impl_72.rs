macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < 'de > serde :: Deserialize < 'de > for SmolStr { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { smol_str (deserializer) } }
    };
}

impl_72!();