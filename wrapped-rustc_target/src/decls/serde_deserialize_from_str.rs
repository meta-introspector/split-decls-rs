macro_rules! serde_deserialize_from_str {
    () => {
        macro_rules ! serde_deserialize_from_str { ($ ty : ty) => { impl <'de > serde :: Deserialize <'de > for $ ty { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer <'de >, { let s = String :: deserialize (deserializer) ?; FromStr :: from_str (& s) . map_err (serde :: de :: Error :: custom) } } } ; }
    };
}

serde_deserialize_from_str!()