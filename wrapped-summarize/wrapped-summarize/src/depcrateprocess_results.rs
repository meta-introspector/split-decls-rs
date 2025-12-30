// Generated macro for process_results (function)
macro_rules! Depcrateprocess_results {
() => {
// Module: crate
// Provides: {"process_results"}
// Dependencies: {}
fn process_results (file : & PathBuf) -> Result < AnalysisResults , Box < dyn Error + Send + Sync > > { if file . ends_with ("json") { let reader = BufReader :: new (File :: open (& file) ?) ; let results : AnalysisResults = serde_json :: from_reader (reader) ? ; Ok (results) } else { let data = ProfilingData :: new (& file) ? ; Ok (data . perform_analysis ()) } }
};
}
