// Generated macro for impl_72 (impl)
macro_rules! Depcrate_filter_boxedimpl_72 {
() => {
// Module: crate::filter::boxed
// Provides: {"impl_72"}
// Dependencies: {}
impl < T : Tuple + Send > BoxedFilter < T > { pub (super) fn new < F > (filter : F) -> BoxedFilter < T > where F : Filter < Extract = T > + Send + Sync + 'static , F :: Error : Into < Rejection > , { BoxedFilter { filter : Arc :: new (BoxingFilter { filter : filter . map_err (super :: Internal , Into :: into) , }) , } } }
};
}
