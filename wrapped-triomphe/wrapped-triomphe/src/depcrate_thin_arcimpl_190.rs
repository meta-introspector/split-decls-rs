// Generated macro for impl_190 (impl)
macro_rules! Depcrate_thin_arcimpl_190 {
() => {
// Module: crate::thin_arc
// Provides: {"impl_190"}
// Dependencies: {}
impl < H : Hash , T : Hash > Hash for ThinArc < H , T > { fn hash < HSR : Hasher > (& self , state : & mut HSR) { ThinArc :: with_arc (self , | a | a . hash (state)) } }
};
}
