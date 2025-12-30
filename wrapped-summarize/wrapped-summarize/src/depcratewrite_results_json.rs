// Generated macro for write_results_json (function)
macro_rules! Depcratewrite_results_json {
() => {
// Module: crate
// Provides: {"write_results_json"}
// Dependencies: {}
fn write_results_json (file : & PathBuf , results : impl Serialize ,) -> Result < () , Box < dyn Error + Send + Sync > > { let file = BufWriter :: new (File :: create (file . with_extension ("json")) ?) ; serde_json :: to_writer (file , & results) ? ; Ok (()) }
};
}
