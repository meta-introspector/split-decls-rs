// Generated macro for is_normalized_windows_drive_letter (function)
macro_rules! Depcrate_parseris_normalized_windows_drive_letter {
() => {
// Module: crate::parser
// Provides: {"is_normalized_windows_drive_letter"}
// Dependencies: {}
fn is_normalized_windows_drive_letter (segment : & str) -> bool { is_windows_drive_letter (segment) && segment . as_bytes () [1] == b':' }
};
}
