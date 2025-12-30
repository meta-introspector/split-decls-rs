// Generated macro for impl_186 (impl)
macro_rules! Depcrate_thin_arcimpl_186 {
() => {
// Module: crate::thin_arc
// Provides: {"impl_186"}
// Dependencies: {}
impl < H : PartialEq , T : PartialEq > PartialEq for ThinArc < H , T > { # [inline] fn eq (& self , other : & ThinArc < H , T >) -> bool { ThinArc :: with_arc (self , | a | ThinArc :: with_arc (other , | b | * a == * b)) } }
};
}
