// Generated macro for impl_94 (impl)
macro_rules! Depcrate_deimpl_94 {
() => {
// Module: crate::de
// Provides: {"impl_94"}
// Dependencies: {}
impl < 'de , T > Deserialize < 'de > for Option < T > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct BytesVisitor < T > { out : PhantomData < T > , } impl < 'de , T > Visitor < 'de > for BytesVisitor < T > where T : Deserialize < 'de > , { type Value = Option < T > ; fn expecting (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str ("optional byte array") } fn visit_unit < E : Error > (self) -> Result < Self :: Value , E > { Ok (None) } fn visit_none < E : Error > (self) -> Result < Self :: Value , E > { Ok (None) } fn visit_some < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { T :: deserialize (deserializer) . map (Some) } } let visitor = BytesVisitor { out : PhantomData } ; deserializer . deserialize_option (visitor) } }
};
}
