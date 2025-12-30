// Generated macro for impl_188 (impl)
macro_rules! Depcrate_thin_arcimpl_188 {
() => {
// Module: crate::thin_arc
// Provides: {"impl_188"}
// Dependencies: {}
impl < H : PartialOrd , T : PartialOrd > PartialOrd for ThinArc < H , T > { # [inline] fn partial_cmp (& self , other : & ThinArc < H , T >) -> Option < Ordering > { ThinArc :: with_arc (self , | a | ThinArc :: with_arc (other , | b | a . partial_cmp (b))) } }
};
}
