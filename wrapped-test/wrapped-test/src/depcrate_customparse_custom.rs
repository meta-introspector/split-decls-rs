// Generated macro for parse_custom (function)
macro_rules! Depcrate_customparse_custom {
() => {
// Module: crate::custom
// Provides: {"parse_custom"}
// Dependencies: {}
fn parse_custom (s : & str) -> Result < (String , String) > { let mut parts = s . splitn (2 , '=') ; Ok ((parts . next () . unwrap () . to_string () , parts . next () . context ("must be of the form `a=b`") ? . to_string () ,)) }
};
}
