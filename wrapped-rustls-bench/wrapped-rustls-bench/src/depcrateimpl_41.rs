// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl LatencyOutput { fn new (prefix : & Option < String > , role : & str) -> Self { let thread_id = thread :: current () . id () ; let output = prefix . as_ref () . map (| prefix | { let file_name = format ! ("{prefix}-{role}-{thread_id:?}-latency.tsv") ; File :: create (& file_name) . expect ("cannot open latency output file") }) ; Self { output } } fn sample (& mut self , secs : f64) { if let Some (file) = & mut self . output { writeln ! (file , "{:.8}\t{:.8}" , wall_time () , secs * 1e6) . unwrap () ; } } }
};
}
