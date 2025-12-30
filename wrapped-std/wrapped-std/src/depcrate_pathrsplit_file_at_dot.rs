// Generated macro for rsplit_file_at_dot (function)
macro_rules! Depcrate_pathrsplit_file_at_dot {
() => {
// Module: crate::path
// Provides: {"rsplit_file_at_dot"}
// Dependencies: {}
fn rsplit_file_at_dot (file : & OsStr) -> (Option < & OsStr > , Option < & OsStr >) { if file . as_encoded_bytes () == b".." { return (Some (file) , None) ; } let mut iter = file . as_encoded_bytes () . rsplitn (2 , | b | * b == b'.') ; let after = iter . next () ; let before = iter . next () ; if before == Some (b"") { (Some (file) , None) } else { unsafe { (before . map (| s | OsStr :: from_encoded_bytes_unchecked (s)) , after . map (| s | OsStr :: from_encoded_bytes_unchecked (s)) ,) } } }
};
}
