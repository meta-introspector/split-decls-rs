// Generated macro for impl_178 (impl)
macro_rules! Depcrate_filter_predicateimpl_178 {
() => {
// Module: crate::filter::predicate
// Provides: {"impl_178"}
// Dependencies: {}
impl < F , T , R , E > Predicate < T > for F where F : FnMut (T) -> Result < R , E > , E : Into < BoxError > , { type Request = R ; fn check (& mut self , request : T) -> Result < Self :: Request , BoxError > { self (request) . map_err (Into :: into) } }
};
}
