macro_rules! deps {
    () => {
        Result!();
        InheritEdition!();
        Error!();
        Edition!();
        EditionOrInherit!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for EditionOrInherit { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct EditionOrInheritVisitor ; impl < 'de > Visitor < 'de > for EditionOrInheritVisitor { type Value = EditionOrInherit ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("edition") } fn visit_str < E > (self , s : & str) -> Result < Self :: Value , E > where E : de :: Error , { Edition :: deserialize (StrDeserializer :: new (s)) . map (EditionOrInherit :: Edition) } fn visit_map < M > (self , map : M) -> Result < Self :: Value , M :: Error > where M : de :: MapAccess < 'de > , { InheritEdition :: deserialize (MapAccessDeserializer :: new (map)) ? ; Ok (EditionOrInherit :: Inherit) } } deserializer . deserialize_any (EditionOrInheritVisitor) } }
    };
}

impl_51!()