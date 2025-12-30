// Generated macro for AcceptHeader (struct)
macro_rules! Depcrate_validate_requestAcceptHeader {
() => {
// Module: crate::validate_request
// Provides: {"AcceptHeader"}
// Dependencies: {}
# [doc = " Type that performs validation of the Accept header."] pub struct AcceptHeader < ResBody > { header_value : Arc < Mime > , _ty : PhantomData < fn () -> ResBody > , }
};
}
