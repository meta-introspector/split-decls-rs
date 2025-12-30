// Generated macro for impl_338 (impl)
macro_rules! Depcrate_varzerovec_componentsimpl_338 {
() => {
// Module: crate::varzerovec::components
// Provides: {"impl_338"}
// Dependencies: {}
impl < 'a , T : VarULE + ? Sized , F : VarZeroVecFormat > ExactSizeIterator for VarZeroSliceIter < 'a , T , F > { fn len (& self) -> usize { self . components . len () - self . index } }
};
}
