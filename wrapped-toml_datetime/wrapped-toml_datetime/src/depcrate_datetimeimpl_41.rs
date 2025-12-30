// Generated macro for impl_41 (impl)
macro_rules! Depcrate_datetimeimpl_41 {
() => {
// Module: crate::datetime
// Provides: {"impl_41"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde_core :: de :: Deserialize < 'de > for DatetimeKey { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde_core :: de :: Deserializer < 'de > , { struct FieldVisitor ; impl serde_core :: de :: Visitor < '_ > for FieldVisitor { type Value = () ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a valid datetime field") } fn visit_str < E > (self , s : & str) -> Result < () , E > where E : serde_core :: de :: Error , { if s == FIELD { Ok (()) } else { Err (serde_core :: de :: Error :: custom ("expected field with custom name" ,)) } } } deserializer . deserialize_identifier (FieldVisitor) ? ; Ok (Self) } }
};
}
