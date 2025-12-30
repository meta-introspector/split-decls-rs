// Generated macro for impl_189 (impl)
macro_rules! Depcrate_thin_arcimpl_189 {
() => {
// Module: crate::thin_arc
// Provides: {"impl_189"}
// Dependencies: {}
impl < H : Ord , T : Ord > Ord for ThinArc < H , T > { # [inline] fn cmp (& self , other : & ThinArc < H , T >) -> Ordering { ThinArc :: with_arc (self , | a | ThinArc :: with_arc (other , | b | a . cmp (b))) } }
};
}
