// Generated macro for impl_160 (impl)
macro_rules! Depcrate_offset_arcimpl_160 {
() => {
// Module: crate::offset_arc
// Provides: {"impl_160"}
// Dependencies: {}
impl < T : PartialEq > PartialEq for OffsetArc < T > { fn eq (& self , other : & OffsetArc < T >) -> bool { * (* self) == * (* other) } # [allow (clippy :: partialeq_ne_impl)] fn ne (& self , other : & OffsetArc < T >) -> bool { * (* self) != * (* other) } }
};
}
