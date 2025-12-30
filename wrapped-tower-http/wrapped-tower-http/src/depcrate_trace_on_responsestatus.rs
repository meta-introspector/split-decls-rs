// Generated macro for status (function)
macro_rules! Depcrate_trace_on_responsestatus {
() => {
// Module: crate::trace::on_response
// Provides: {"status"}
// Dependencies: {}
fn status < B > (res : & Response < B >) -> Option < i32 > { use crate :: classify :: grpc_errors_as_failures :: ParsedGrpcStatus ; let is_grpc = res . headers () . get (http :: header :: CONTENT_TYPE) . map_or (false , | value | { value . as_bytes () . starts_with ("application/grpc" . as_bytes ()) }) ; if is_grpc { match crate :: classify :: grpc_errors_as_failures :: classify_grpc_metadata (res . headers () , crate :: classify :: GrpcCode :: Ok . into_bitmask () ,) { ParsedGrpcStatus :: Success | ParsedGrpcStatus :: HeaderNotString | ParsedGrpcStatus :: HeaderNotInt => Some (0) , ParsedGrpcStatus :: NonSuccess (status) => Some (status . get ()) , ParsedGrpcStatus :: GrpcStatusHeaderMissing => None , } } else { Some (res . status () . as_u16 () . into ()) } }
};
}
