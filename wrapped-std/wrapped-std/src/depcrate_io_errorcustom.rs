// Generated macro for Custom (struct)
macro_rules! Depcrate_io_errorCustom {
() => {
// Module: crate::io::error
// Provides: {"Custom"}
// Dependencies: {}
# [derive (Debug)] # [repr (align (4))] struct Custom { kind : ErrorKind , error : Box < dyn error :: Error + Send + Sync > , }
};
}
