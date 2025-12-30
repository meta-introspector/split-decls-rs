// Generated macro for impl_210 (impl)
macro_rules! Depcrate_de_implsimpl_210 {
() => {
// Module: crate::de::impls
// Provides: {"impl_210"}
// Dependencies: {}
impl < 'de , T > Visitor < 'de > for OptionVisitor < T > where T : Deserialize < 'de > , { type Value = Option < T > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("option") } # [inline] fn visit_unit < E > (self) -> Result < Self :: Value , E > where E : Error , { Ok (None) } # [inline] fn visit_none < E > (self) -> Result < Self :: Value , E > where E : Error , { Ok (None) } # [inline] fn visit_some < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { T :: deserialize (deserializer) . map (Some) } fn __private_visit_untagged_option < D > (self , deserializer : D) -> Result < Self :: Value , () > where D : Deserializer < 'de > , { Ok (T :: deserialize (deserializer) . ok ()) } }
};
}
