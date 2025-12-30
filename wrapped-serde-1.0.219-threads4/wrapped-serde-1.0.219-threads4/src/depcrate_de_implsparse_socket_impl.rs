// Generated macro for parse_socket_impl (macro)
macro_rules! Depcrate_de_implsparse_socket_impl {
() => {
// Module: crate::de::impls
// Provides: {"parse_socket_impl"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (no_core_net)))] macro_rules ! parse_socket_impl { ($ ty : ty , $ expecting : tt , $ new : expr ,) => { impl <'de > Deserialize <'de > for $ ty { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer <'de >, { if deserializer . is_human_readable () { deserializer . deserialize_str (FromStrVisitor :: new ($ expecting)) } else { < (_ , u16) >:: deserialize (deserializer) . map ($ new) } } } } ; }
};
}
