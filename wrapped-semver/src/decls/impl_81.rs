macro_rules! deps {
    () => {
        Version!();
        Error!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for Version { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct VersionVisitor ; impl < 'de > Visitor < 'de > for VersionVisitor { type Value = Version ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("semver version") } fn visit_str < E > (self , string : & str) -> Result < Self :: Value , E > where E : Error , { string . parse () . map_err (Error :: custom) } } deserializer . deserialize_str (VersionVisitor) } }
    };
}

impl_81!()