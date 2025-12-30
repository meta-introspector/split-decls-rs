// Generated macro for check_file_name (function)
macro_rules! Depcrate_publishcheck_file_name {
() => {
// Module: crate::publish
// Provides: {"check_file_name"}
// Dependencies: {}
fn check_file_name < P : AsRef < std :: path :: Path > > (path : P) -> anyhow :: Result < String > { let file_name = path . as_ref () . file_name () . ok_or_else (| | anyhow :: format_err ! ("file name is not specified as `changelog`")) ? . to_string_lossy () ; let mut chars = file_name . chars () ; if file_name . len () >= 10 && chars . next () . unwrap () . is_ascii_digit () && chars . next () . unwrap () . is_ascii_digit () && chars . next () . unwrap () . is_ascii_digit () && chars . next () . unwrap () . is_ascii_digit () && chars . next () . unwrap () == '-' && chars . next () . unwrap () . is_ascii_digit () && chars . next () . unwrap () . is_ascii_digit () && chars . next () . unwrap () == '-' && chars . next () . unwrap () . is_ascii_digit () && chars . next () . unwrap () . is_ascii_digit () { Ok (file_name . to_string ()) } else { bail ! ("unexpected file name format; no date information prefixed") } }
};
}
