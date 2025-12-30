// Generated macro for check_modified_headers (function)
macro_rules! Depcrate_services_fs_serve_dir_open_filecheck_modified_headers {
() => {
// Module: crate::services::fs::serve_dir::open_file
// Provides: {"check_modified_headers"}
// Dependencies: {}
fn check_modified_headers (modified : Option < & LastModified > , if_unmodified_since : Option < IfUnmodifiedSince > , if_modified_since : Option < IfModifiedSince > ,) -> Option < OpenFileOutput > { if let Some (since) = if_unmodified_since { let precondition = modified . as_ref () . map (| time | since . precondition_passes (time)) . unwrap_or (false) ; if ! precondition { return Some (OpenFileOutput :: PreconditionFailed) ; } } if let Some (since) = if_modified_since { let unmodified = modified . as_ref () . map (| time | ! since . is_modified (time)) . unwrap_or (false) ; if unmodified { return Some (OpenFileOutput :: NotModified) ; } } None }
};
}
