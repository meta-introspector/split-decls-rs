// Generated macro for classify_grpc_metadata (function)
macro_rules! Depcrate_classify_grpc_errors_as_failuresclassify_grpc_metadata {
() => {
// Module: crate::classify::grpc_errors_as_failures
// Provides: {"classify_grpc_metadata"}
// Dependencies: {}
pub (crate) fn classify_grpc_metadata (headers : & HeaderMap , success_codes : GrpcCodeBitmask ,) -> ParsedGrpcStatus { macro_rules ! or_else { ($ expr : expr , $ other : ident) => { if let Some (value) = $ expr { value } else { return ParsedGrpcStatus ::$ other ; } } ; } let status = or_else ! (headers . get ("grpc-status") , GrpcStatusHeaderMissing) ; let status = or_else ! (status . to_str () . ok () , HeaderNotString) ; let status = or_else ! (status . parse ::< i32 > () . ok () , HeaderNotInt) ; if GrpcCodeBitmask :: try_from_u32 (status as _) . filter (| code | success_codes . contains (* code)) . is_some () { ParsedGrpcStatus :: Success } else { ParsedGrpcStatus :: NonSuccess (NonZeroI32 :: new (status) . unwrap ()) } }
};
}
