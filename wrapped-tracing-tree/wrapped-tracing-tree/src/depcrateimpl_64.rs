// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl < L : Iterator < Item = T > , R : Iterator < Item = T > , T , U : PartialEq , F : Fn (& T) -> U > DifferenceIter < L , R , F > { fn new (left : L , right : R , compare : F) -> Self { Self { left : left . fuse () , right , compare , } } }
};
}
