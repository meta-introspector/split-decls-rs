// Generated macro for parse_dep_info (function)
macro_rules! Depcrate_compiler_rustparse_dep_info {
() => {
// Module: crate::compiler::rust
// Provides: {"parse_dep_info"}
// Dependencies: {}
fn parse_dep_info < T > (dep_info : & str , cwd : T) -> Vec < PathBuf > where T : AsRef < Path > , { let cwd = cwd . as_ref () ; let line = match dep_info . lines () . next () { None => return vec ! [] , Some (l) => l , } ; let pos = match line . find (": ") { None => return vec ! [] , Some (p) => p , } ; let mut deps = Vec :: new () ; let mut current_dep = String :: new () ; let mut iter = line [pos + 2 ..] . chars () . peekable () ; loop { match iter . next () { Some ('\\') => { if iter . peek () == Some (& ' ') { current_dep . push (' ') ; iter . next () ; } else { current_dep . push ('\\') ; } } Some (' ') => { deps . push (current_dep) ; current_dep = String :: new () ; } Some (c) => current_dep . push (c) , None => { if ! current_dep . is_empty () { deps . push (current_dep) ; } break ; } } } let mut deps = deps . iter () . map (| s | cwd . join (s)) . collect :: < Vec < _ > > () ; deps . sort () ; deps }
};
}
