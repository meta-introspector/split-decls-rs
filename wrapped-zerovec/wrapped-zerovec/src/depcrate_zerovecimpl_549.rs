// Generated macro for impl_549 (impl)
macro_rules! Depcrate_zerovecimpl_549 {
() => {
// Module: crate::zerovec
// Provides: {"impl_549"}
// Dependencies: {}
impl < 'a , 'b , T > PartialEq < ZeroVec < 'b , T > > for ZeroVec < 'a , T > where T : AsULE + PartialEq , { # [inline] fn eq (& self , other : & ZeroVec < 'b , T >) -> bool { self . iter () . eq (other . iter ()) } }
};
}
