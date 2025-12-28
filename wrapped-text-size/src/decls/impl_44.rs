macro_rules! deps {
    () => {
        TextSize!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for TextSize { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { u32 :: deserialize (deserializer) . map (TextSize :: from) } }
    };
}

impl_44!();