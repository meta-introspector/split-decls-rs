// Generated macro for impl_803 (impl)
macro_rules! Depcrate_specimpl_803 {
() => {
// Module: crate::spec
// Provides: {"impl_803"}
// Dependencies: {}
impl Hash for TargetTuple { fn hash < H : Hasher > (& self , state : & mut H) -> () { match self { TargetTuple :: TargetTuple (tuple) => { 0u8 . hash (state) ; tuple . hash (state) } TargetTuple :: TargetJson { path_for_rustdoc : _ , tuple , contents } => { 1u8 . hash (state) ; tuple . hash (state) ; contents . hash (state) } } } }
};
}
