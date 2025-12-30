// Generated macro for parse_dep_file (function)
macro_rules! Depcrate_compiler_rustparse_dep_file {
() => {
// Module: crate::compiler::rust
// Provides: {"parse_dep_file"}
// Dependencies: {}
# [doc = " Parse dependency info from `file` and return a Vec of files mentioned."] # [doc = " Treat paths as relative to `cwd`."] fn parse_dep_file < T , U > (file : T , cwd : U) -> Result < (Vec < PathBuf > , Vec < (OsString , OsString) >) > where T : AsRef < Path > , U : AsRef < Path > , { let mut f = fs :: File :: open (file . as_ref ()) ? ; let mut deps = String :: new () ; f . read_to_string (& mut deps) ? ; Ok ((parse_dep_info (& deps , cwd) , parse_env_dep_info (& deps))) }
};
}
