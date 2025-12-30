// Generated macro for atomic_impl (macro)
macro_rules! Depcrate_ser_implsatomic_impl {
() => {
// Module: crate::ser::impls
// Provides: {"atomic_impl"}
// Dependencies: {}
# [cfg (all (feature = "std" , not (no_std_atomic)))] macro_rules ! atomic_impl { ($ ($ ty : ident $ size : expr) *) => { $ (# [cfg (any (no_target_has_atomic , target_has_atomic = $ size))] # [cfg_attr (docsrs , doc (cfg (all (feature = "std" , target_has_atomic = $ size))))] impl Serialize for $ ty { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . load (Ordering :: Relaxed) . serialize (serializer) } }) * } }
};
}
