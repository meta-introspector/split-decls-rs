// Generated macro for impl_1024 (impl)
macro_rules! Depcrate_classify_grpc_errors_as_failuresimpl_1024 {
() => {
// Module: crate::classify::grpc_errors_as_failures
// Provides: {"impl_1024"}
// Dependencies: {}
impl GrpcCodeBitmask { fn try_from_u32 (code : u32) -> Option < Self > { match code { 0 => Some (Self :: OK) , 1 => Some (Self :: CANCELLED) , 2 => Some (Self :: UNKNOWN) , 3 => Some (Self :: INVALID_ARGUMENT) , 4 => Some (Self :: DEADLINE_EXCEEDED) , 5 => Some (Self :: NOT_FOUND) , 6 => Some (Self :: ALREADY_EXISTS) , 7 => Some (Self :: PERMISSION_DENIED) , 8 => Some (Self :: RESOURCE_EXHAUSTED) , 9 => Some (Self :: FAILED_PRECONDITION) , 10 => Some (Self :: ABORTED) , 11 => Some (Self :: OUT_OF_RANGE) , 12 => Some (Self :: UNIMPLEMENTED) , 13 => Some (Self :: INTERNAL) , 14 => Some (Self :: UNAVAILABLE) , 15 => Some (Self :: DATA_LOSS) , 16 => Some (Self :: UNAUTHENTICATED) , _ => None , } } }
};
}
