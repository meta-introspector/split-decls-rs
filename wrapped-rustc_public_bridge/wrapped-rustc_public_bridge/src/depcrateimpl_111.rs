// Generated macro for impl_111 (impl)
macro_rules! Depcrateimpl_111 {
() => {
// Module: crate
// Provides: {"impl_111"}
// Dependencies: {}
impl < 'tcx , B : Bridge > Index < B :: DefId > for Tables < 'tcx , B > { type Output = DefId ; # [inline (always)] fn index (& self , index : B :: DefId) -> & Self :: Output { & self . def_ids [index] } }
};
}
