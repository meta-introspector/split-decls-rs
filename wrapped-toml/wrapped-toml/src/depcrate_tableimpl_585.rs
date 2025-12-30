// Generated macro for impl_585 (impl)
macro_rules! Depcrate_tableimpl_585 {
() => {
// Module: crate::table
// Provides: {"impl_585"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for Table { # [inline] fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { struct Visitor ; impl < 'de > de :: Visitor < 'de > for Visitor { type Value = Map < String , Value > ; fn expecting (& self , formatter : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { formatter . write_str ("a map") } # [inline] fn visit_unit < E > (self) -> Result < Self :: Value , E > where E : de :: Error , { Ok (Map :: new ()) } # [inline] fn visit_map < V > (self , mut visitor : V) -> Result < Self :: Value , V :: Error > where V : de :: MapAccess < 'de > , { let mut values = Map :: new () ; while let Some ((key , value)) = visitor . next_entry () ? { values . insert (key , value) ; } Ok (values) } } deserializer . deserialize_map (Visitor) } }
};
}
