// Generated macro for impl_512 (impl)
macro_rules! Depcrate_filters_pathimpl_512 {
() => {
// Module: crate::filters::path
// Provides: {"impl_512"}
// Dependencies: {}
impl < P > FilterBase for Exact < P > where P : AsRef < str > , { type Extract = () ; type Error = Rejection ; type Future = future :: Ready < Result < Self :: Extract , Self :: Error > > ; # [inline] fn filter (& self , _ : Internal) -> Self :: Future { route :: with (| route | { let p = self . 0 . as_ref () ; future :: ready (with_segment (route , | seg | { tracing :: trace ! ("{:?}?: {:?}" , p , seg) ; if seg == p { Ok (()) } else { Err (reject :: not_found ()) } })) }) } }
};
}
