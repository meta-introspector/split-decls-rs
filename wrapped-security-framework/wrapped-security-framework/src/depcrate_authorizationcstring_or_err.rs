// Generated macro for cstring_or_err (macro)
macro_rules! Depcrate_authorizationcstring_or_err {
() => {
// Module: crate::authorization
// Provides: {"cstring_or_err"}
// Dependencies: {}
macro_rules ! cstring_or_err { ($ x : expr) => { { CString :: new ($ x) . map_err (| _ | Error :: from_code (errSecConversionError)) } } ; }
};
}
