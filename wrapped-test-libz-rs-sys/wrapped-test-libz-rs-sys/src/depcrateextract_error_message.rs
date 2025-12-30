// Generated macro for extract_error_message (macro)
macro_rules! Depcrateextract_error_message {
() => {
// Module: crate
// Provides: {"extract_error_message"}
// Dependencies: {}
# [cfg (test)] # [allow (unused)] macro_rules ! extract_error_message { ($ strm : expr) => { if !$ strm . msg . is_null () { core :: ffi :: CStr :: from_ptr ($ strm . msg) . to_str () } else { Ok ("NULL") } } ; }
};
}
