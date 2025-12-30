// Generated macro for impl_181 (impl)
macro_rules! Depcrate_thin_arcimpl_181 {
() => {
// Module: crate::thin_arc
// Provides: {"impl_181"}
// Dependencies: {}
impl < H , T > Deref for ThinArc < H , T > { type Target = HeaderSliceWithLengthUnchecked < H , T > ; # [inline] fn deref (& self) -> & Self :: Target { unsafe { (* thin_to_thick (self)) . data . inner () } } }
};
}
