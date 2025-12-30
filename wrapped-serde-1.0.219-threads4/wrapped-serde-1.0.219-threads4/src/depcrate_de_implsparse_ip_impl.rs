// Generated macro for parse_ip_impl (macro)
macro_rules! Depcrate_de_implsparse_ip_impl {
() => {
// Module: crate::de::impls
// Provides: {"parse_ip_impl"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (no_core_net)))] macro_rules ! parse_ip_impl { ($ ty : ty , $ expecting : expr , $ size : tt) => { impl <'de > Deserialize <'de > for $ ty { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer <'de >, { if deserializer . is_human_readable () { deserializer . deserialize_str (FromStrVisitor :: new ($ expecting)) } else { < [u8 ; $ size] >:: deserialize (deserializer) . map (<$ ty >:: from) } } } } ; }
};
}
