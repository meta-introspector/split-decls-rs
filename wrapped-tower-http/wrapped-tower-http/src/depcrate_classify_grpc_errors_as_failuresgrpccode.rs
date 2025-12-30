// Generated macro for GrpcCode (enum)
macro_rules! Depcrate_classify_grpc_errors_as_failuresGrpcCode {
() => {
// Module: crate::classify::grpc_errors_as_failures
// Provides: {"GrpcCode"}
// Dependencies: {}
# [doc = " gRPC status codes."] # [doc = ""] # [doc = " These variants match the [gRPC status codes]."] # [doc = ""] # [doc = " [gRPC status codes]: https://github.com/grpc/grpc/blob/master/doc/statuscodes.md#status-codes-and-their-use-in-grpc"] # [derive (Clone , Copy , Debug)] pub enum GrpcCode { # [doc = " The operation completed successfully."] Ok , # [doc = " The operation was cancelled."] Cancelled , # [doc = " Unknown error."] Unknown , # [doc = " Client specified an invalid argument."] InvalidArgument , # [doc = " Deadline expired before operation could complete."] DeadlineExceeded , # [doc = " Some requested entity was not found."] NotFound , # [doc = " Some entity that we attempted to create already exists."] AlreadyExists , # [doc = " The caller does not have permission to execute the specified operation."] PermissionDenied , # [doc = " Some resource has been exhausted."] ResourceExhausted , # [doc = " The system is not in a state required for the operation's execution."] FailedPrecondition , # [doc = " The operation was aborted."] Aborted , # [doc = " Operation was attempted past the valid range."] OutOfRange , # [doc = " Operation is not implemented or not supported."] Unimplemented , # [doc = " Internal error."] Internal , # [doc = " The service is currently unavailable."] Unavailable , # [doc = " Unrecoverable data loss or corruption."] DataLoss , # [doc = " The request does not have valid authentication credentials"] Unauthenticated , }
};
}
