// Generated macro for validate_extension (function)
macro_rules! Depcrate_pathvalidate_extension {
() => {
// Module: crate::path
// Provides: {"validate_extension"}
// Dependencies: {}
# [doc = " Checks whether the string is valid as a file extension, or panics otherwise."] fn validate_extension (extension : & OsStr) { for & b in extension . as_encoded_bytes () { if is_sep_byte (b) { panic ! ("extension cannot contain path separators: {extension:?}") ; } } }
};
}
