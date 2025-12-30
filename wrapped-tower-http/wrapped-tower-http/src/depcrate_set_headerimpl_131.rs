// Generated macro for impl_131 (impl)
macro_rules! Depcrate_set_headerimpl_131 {
() => {
// Module: crate::set_header
// Provides: {"impl_131"}
// Dependencies: {}
impl InsertHeaderMode { fn apply < T , M > (self , header_name : & HeaderName , target : & mut T , make : & mut M) where T : Headers , M : MakeHeaderValue < T > , { match self { InsertHeaderMode :: Override => { if let Some (value) = make . make_header_value (target) { target . headers_mut () . insert (header_name . clone () , value) ; } } InsertHeaderMode :: IfNotPresent => { if ! target . headers () . contains_key (header_name) { if let Some (value) = make . make_header_value (target) { target . headers_mut () . insert (header_name . clone () , value) ; } } } InsertHeaderMode :: Append => { if let Some (value) = make . make_header_value (target) { target . headers_mut () . append (header_name . clone () , value) ; } } } } }
};
}
