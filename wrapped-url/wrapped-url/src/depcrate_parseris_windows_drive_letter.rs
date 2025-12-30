// Generated macro for is_windows_drive_letter (function)
macro_rules! Depcrate_parseris_windows_drive_letter {
() => {
// Module: crate::parser
// Provides: {"is_windows_drive_letter"}
// Dependencies: {}
# [doc = " Whether the scheme is file:, the path has a single segment, and that segment"] # [doc = " is a Windows drive letter"] # [inline] pub fn is_windows_drive_letter (segment : & str) -> bool { segment . len () == 2 && starts_with_windows_drive_letter (segment) }
};
}
