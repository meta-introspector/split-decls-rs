// Generated macro for impl_289 (impl)
macro_rules! Depcrate_de_implsimpl_289 {
() => {
// Module: crate::de::impls
// Provides: {"impl_289"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (no_core_net)))] impl < 'de , T > Visitor < 'de > for FromStrVisitor < T > where T : str :: FromStr , T :: Err : fmt :: Display , { type Value = T ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str (self . expecting) } fn visit_str < E > (self , s : & str) -> Result < Self :: Value , E > where E : Error , { s . parse () . map_err (Error :: custom) } }
};
}
