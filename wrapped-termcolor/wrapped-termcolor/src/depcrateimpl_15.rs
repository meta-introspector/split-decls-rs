// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl ColorChoice { # [doc = " Returns true if we should attempt to write colored output."] fn should_attempt_color (& self) -> bool { match * self { ColorChoice :: Always => true , ColorChoice :: AlwaysAnsi => true , ColorChoice :: Never => false , ColorChoice :: Auto => self . env_allows_color () , } } # [cfg (not (windows))] fn env_allows_color (& self) -> bool { match env :: var_os ("TERM") { None => return false , Some (k) => { if k == "dumb" { return false ; } } } if env :: var_os ("NO_COLOR") . is_some () { return false ; } true } # [cfg (windows)] fn env_allows_color (& self) -> bool { if let Some (k) = env :: var_os ("TERM") { if k == "dumb" { return false ; } } if env :: var_os ("NO_COLOR") . is_some () { return false ; } true } # [doc = " Returns true if this choice should forcefully use ANSI color codes."] # [doc = ""] # [doc = " It's possible that ANSI is still the correct choice even if this"] # [doc = " returns false."] # [cfg (windows)] fn should_ansi (& self) -> bool { match * self { ColorChoice :: Always => false , ColorChoice :: AlwaysAnsi => true , ColorChoice :: Never => false , ColorChoice :: Auto => { match env :: var ("TERM") { Err (_) => false , Ok (k) => k != "dumb" && k != "cygwin" , } } } } }
};
}
