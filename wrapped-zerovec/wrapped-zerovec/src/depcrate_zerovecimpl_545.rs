// Generated macro for impl_545 (impl)
macro_rules! Depcrate_zerovecimpl_545 {
() => {
// Module: crate::zerovec
// Provides: {"impl_545"}
// Dependencies: {}
impl < 'a , T : AsULE > Clone for ZeroVec < 'a , T > { fn clone (& self) -> Self { # [cfg (feature = "alloc")] if self . is_owned () { return ZeroVec :: new_owned (self . as_ule_slice () . into ()) ; } Self { vector : EyepatchHackVector { buf : self . vector . buf , # [cfg (feature = "alloc")] capacity : 0 , } , marker1 : PhantomData , marker2 : PhantomData , } } }
};
}
