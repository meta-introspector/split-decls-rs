macro_rules! deps {
    () => {
        Error!();
        Comparator!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for Comparator { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct ComparatorVisitor ; impl < 'de > Visitor < 'de > for ComparatorVisitor { type Value = Comparator ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("semver comparator") } fn visit_str < E > (self , string : & str) -> Result < Self :: Value , E > where E : Error , { string . parse () . map_err (Error :: custom) } } deserializer . deserialize_str (ComparatorVisitor) } }
    };
}

impl_83!()