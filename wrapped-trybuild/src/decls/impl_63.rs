macro_rules! deps {
    () => {
        Result!();
        Error!();
        Directory!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for Directory { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { PathBuf :: deserialize (deserializer) . map (Directory :: new) } }
    };
}

impl_63!();