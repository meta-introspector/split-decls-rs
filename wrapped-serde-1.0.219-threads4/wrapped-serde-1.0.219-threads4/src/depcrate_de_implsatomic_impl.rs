// Generated macro for atomic_impl (macro)
macro_rules! Depcrate_de_implsatomic_impl {
() => {
// Module: crate::de::impls
// Provides: {"atomic_impl"}
// Dependencies: {}
# [cfg (all (feature = "std" , not (no_std_atomic)))] macro_rules ! atomic_impl { ($ ($ ty : ident $ size : expr) *) => { $ (# [cfg (any (no_target_has_atomic , target_has_atomic = $ size))] # [cfg_attr (docsrs , doc (cfg (all (feature = "std" , target_has_atomic = $ size))))] impl <'de > Deserialize <'de > for $ ty { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer <'de >, { Deserialize :: deserialize (deserializer) . map (Self :: new) } }) * } ; }
};
}
