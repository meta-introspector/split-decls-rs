// Generated macro for impl_177 (impl)
macro_rules! Depcrate_filter_predicateimpl_177 {
() => {
// Module: crate::filter::predicate
// Provides: {"impl_177"}
// Dependencies: {}
impl < F , T , U , R , E > AsyncPredicate < T > for F where F : FnMut (T) -> U , U : Future < Output = Result < R , E > > , E : Into < BoxError > , { type Future = futures_util :: future :: ErrInto < U , BoxError > ; type Request = R ; fn check (& mut self , request : T) -> Self :: Future { use futures_util :: TryFutureExt ; self (request) . err_into () } }
};
}
