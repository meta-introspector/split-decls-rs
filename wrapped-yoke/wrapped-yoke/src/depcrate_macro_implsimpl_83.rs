// Generated macro for impl_83 (impl)
macro_rules! Depcrate_macro_implsimpl_83 {
() => {
// Module: crate::macro_impls
// Provides: {"impl_83"}
// Dependencies: {}
unsafe impl < 'a , T1 : 'static + for < 'b > Yokeable < 'b > , T2 : 'static + for < 'b > Yokeable < 'b > > Yokeable < 'a > for (T1 , T2) { type Output = (< T1 as Yokeable < 'a > > :: Output , < T2 as Yokeable < 'a > > :: Output) ; unsafe_complex_yoke_impl ! () ; }
};
}
