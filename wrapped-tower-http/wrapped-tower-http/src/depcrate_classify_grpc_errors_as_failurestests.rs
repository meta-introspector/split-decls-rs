// Generated macro for tests (module)
macro_rules! Depcrate_classify_grpc_errors_as_failurestests {
() => {
// Module: crate::classify::grpc_errors_as_failures
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; macro_rules ! classify_grpc_metadata_test { (name : $ name : ident , status : $ status : expr , success_flags : $ success_flags : expr , expected : $ expected : expr ,) => { # [test] fn $ name () { let mut headers = HeaderMap :: new () ; headers . insert ("grpc-status" , $ status . parse () . unwrap ()) ; let status = classify_grpc_metadata (& headers , $ success_flags) ; assert_eq ! (status , $ expected) ; } } ; } classify_grpc_metadata_test ! { name : basic_ok , status : "0" , success_flags : GrpcCodeBitmask :: OK , expected : ParsedGrpcStatus :: Success , } classify_grpc_metadata_test ! { name : basic_error , status : "1" , success_flags : GrpcCodeBitmask :: OK , expected : ParsedGrpcStatus :: NonSuccess (NonZeroI32 :: new (1) . unwrap ()) , } classify_grpc_metadata_test ! { name : two_success_codes_first_matches , status : "0" , success_flags : GrpcCodeBitmask :: OK | GrpcCodeBitmask :: INVALID_ARGUMENT , expected : ParsedGrpcStatus :: Success , } classify_grpc_metadata_test ! { name : two_success_codes_second_matches , status : "3" , success_flags : GrpcCodeBitmask :: OK | GrpcCodeBitmask :: INVALID_ARGUMENT , expected : ParsedGrpcStatus :: Success , } classify_grpc_metadata_test ! { name : two_success_codes_none_matches , status : "16" , success_flags : GrpcCodeBitmask :: OK | GrpcCodeBitmask :: INVALID_ARGUMENT , expected : ParsedGrpcStatus :: NonSuccess (NonZeroI32 :: new (16) . unwrap ()) , } }
};
}
