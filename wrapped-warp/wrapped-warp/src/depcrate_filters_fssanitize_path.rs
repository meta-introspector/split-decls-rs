// Generated macro for sanitize_path (function)
macro_rules! Depcrate_filters_fssanitize_path {
() => {
// Module: crate::filters::fs
// Provides: {"sanitize_path"}
// Dependencies: {}
fn sanitize_path (base : impl AsRef < Path > , tail : & str) -> Result < PathBuf , Rejection > { let mut buf = PathBuf :: from (base . as_ref ()) ; let p = match percent_decode_str (tail) . decode_utf8 () { Ok (p) => p , Err (err) => { tracing :: debug ! ("dir: failed to decode route={:?}: {:?}" , tail , err) ; return Err (reject :: not_found ()) ; } } ; tracing :: trace ! ("dir? base={:?}, route={:?}" , base . as_ref () , p) ; for seg in p . split ('/') { if seg . starts_with ("..") { tracing :: warn ! ("dir: rejecting segment starting with '..'") ; return Err (reject :: not_found ()) ; } else if seg . contains ('\\') { tracing :: warn ! ("dir: rejecting segment containing backslash (\\)") ; return Err (reject :: not_found ()) ; } else if cfg ! (windows) && seg . contains (':') { tracing :: warn ! ("dir: rejecting segment containing colon (:)") ; return Err (reject :: not_found ()) ; } else { buf . push (seg) ; } } Ok (buf) }
};
}
