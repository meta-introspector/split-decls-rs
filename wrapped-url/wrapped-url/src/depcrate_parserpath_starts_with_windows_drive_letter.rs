// Generated macro for path_starts_with_windows_drive_letter (function)
macro_rules! Depcrate_parserpath_starts_with_windows_drive_letter {
() => {
// Module: crate::parser
// Provides: {"path_starts_with_windows_drive_letter"}
// Dependencies: {}
# [doc = " Whether path starts with a root slash"] # [doc = " and a windows drive letter eg: \"/c:\" or \"/a:/\""] fn path_starts_with_windows_drive_letter (s : & str) -> bool { if let Some (c) = s . as_bytes () . first () { matches ! (c , b'/' | b'\\' | b'?' | b'#') && starts_with_windows_drive_letter (& s [1 ..]) } else { false } }
};
}
