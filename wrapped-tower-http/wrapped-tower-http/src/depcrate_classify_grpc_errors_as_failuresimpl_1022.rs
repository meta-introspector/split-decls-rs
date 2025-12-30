// Generated macro for impl_1022 (impl)
macro_rules! Depcrate_classify_grpc_errors_as_failuresimpl_1022 {
() => {
// Module: crate::classify::grpc_errors_as_failures
// Provides: {"impl_1022"}
// Dependencies: {}
impl GrpcCode { pub (crate) fn into_bitmask (self) -> GrpcCodeBitmask { match self { Self :: Ok => GrpcCodeBitmask :: OK , Self :: Cancelled => GrpcCodeBitmask :: CANCELLED , Self :: Unknown => GrpcCodeBitmask :: UNKNOWN , Self :: InvalidArgument => GrpcCodeBitmask :: INVALID_ARGUMENT , Self :: DeadlineExceeded => GrpcCodeBitmask :: DEADLINE_EXCEEDED , Self :: NotFound => GrpcCodeBitmask :: NOT_FOUND , Self :: AlreadyExists => GrpcCodeBitmask :: ALREADY_EXISTS , Self :: PermissionDenied => GrpcCodeBitmask :: PERMISSION_DENIED , Self :: ResourceExhausted => GrpcCodeBitmask :: RESOURCE_EXHAUSTED , Self :: FailedPrecondition => GrpcCodeBitmask :: FAILED_PRECONDITION , Self :: Aborted => GrpcCodeBitmask :: ABORTED , Self :: OutOfRange => GrpcCodeBitmask :: OUT_OF_RANGE , Self :: Unimplemented => GrpcCodeBitmask :: UNIMPLEMENTED , Self :: Internal => GrpcCodeBitmask :: INTERNAL , Self :: Unavailable => GrpcCodeBitmask :: UNAVAILABLE , Self :: DataLoss => GrpcCodeBitmask :: DATA_LOSS , Self :: Unauthenticated => GrpcCodeBitmask :: UNAUTHENTICATED , } } }
};
}
