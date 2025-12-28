macro_rules! deps {
    () => {
        JoinedArgs!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < 'de > serde :: de :: Deserialize < 'de > for JoinedArgs { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: de :: Deserializer < 'de > , { let s = String :: deserialize (deserializer) ? ; std :: str :: FromStr :: from_str (& s) . map_err (serde :: de :: Error :: custom) } }
    };
}

impl_23!()