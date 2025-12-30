// Generated macro for impl_37 (impl)
macro_rules! Depcrate_datetimeimpl_37 {
() => {
// Module: crate::datetime
// Provides: {"impl_37"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde_core :: de :: Deserialize < 'de > for Datetime { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde_core :: de :: Deserializer < 'de > , { struct DatetimeVisitor ; impl < 'de > serde_core :: de :: Visitor < 'de > for DatetimeVisitor { type Value = Datetime ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a TOML datetime") } fn visit_map < V > (self , mut visitor : V) -> Result < Datetime , V :: Error > where V : serde_core :: de :: MapAccess < 'de > , { let value = visitor . next_key :: < DatetimeKey > () ? ; if value . is_none () { return Err (serde_core :: de :: Error :: custom ("datetime key not found")) ; } let v : DatetimeFromString = visitor . next_value () ? ; Ok (v . value) } } static FIELDS : [& str ; 1] = [FIELD] ; deserializer . deserialize_struct (NAME , & FIELDS , DatetimeVisitor) } }
};
}
