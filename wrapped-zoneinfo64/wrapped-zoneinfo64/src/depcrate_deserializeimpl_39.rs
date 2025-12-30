// Generated macro for impl_39 (impl)
macro_rules! Depcrate_deserializeimpl_39 {
() => {
// Module: crate::deserialize
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'de : 'a , 'a > Deserialize < 'de > for TzZoneRaw < 'a > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { struct TzDataRuleEnumVisitor < 'a > { phantom : PhantomData < TzZoneRaw < 'a > > , } impl < 'de : 'a , 'a > Visitor < 'de > for TzDataRuleEnumVisitor < 'a > { type Value = TzZoneRaw < 'a > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("an unsigned 32-bit integer or a table of rule data") } fn visit_u32 < E > (self , v : u32) -> Result < Self :: Value , E > where E : Error , { Ok (TzZoneRaw :: Int (v)) } fn visit_map < A > (self , map : A) -> Result < Self :: Value , A :: Error > where A : MapAccess < 'de > , { let value = TzZoneDataRaw :: deserialize (value :: MapAccessDeserializer :: new (map)) ? ; Ok (TzZoneRaw :: Table (value)) } } deserializer . deserialize_any (TzDataRuleEnumVisitor { phantom : PhantomData , }) } }
};
}
