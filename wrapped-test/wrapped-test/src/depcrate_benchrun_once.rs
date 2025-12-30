// Generated macro for run_once (function)
macro_rules! Depcrate_benchrun_once {
() => {
// Module: crate::bench
// Provides: {"run_once"}
// Dependencies: {}
pub fn run_once < F > (f : F) -> Result < () , String > where F : FnMut (& mut Bencher) -> Result < () , String > , { let mut bs = Bencher { mode : BenchMode :: Single , summary : None , bytes : 0 } ; bs . bench (f) . map (| _ | ()) }
};
}
