// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { let config = Config :: from_env () ; let mut buffer = vec ! [0 ; config . buffer_count * config . buffer_size] ; for path in env :: args_os () . skip (1) { let path = PathBuf :: from (path) ; let hash = hash_one_file (& config , & path , & mut buffer) ? ; eprintln ! ("{hash:x}  {}" , path . display ()) ; } Ok (()) }
};
}
