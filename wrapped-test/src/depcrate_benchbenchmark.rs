// Generated macro for benchmark (function)
macro_rules! Depcrate_benchbenchmark {
() => {
// Module: crate::bench
// Provides: {"benchmark"}
// Dependencies: {}
pub fn benchmark < F > (id : TestId , desc : TestDesc , monitor_ch : Sender < CompletedTest > , nocapture : bool , f : F ,) where F : FnMut (& mut Bencher) -> Result < () , String > , { let mut bs = Bencher { mode : BenchMode :: Auto , summary : None , bytes : 0 } ; let data = Arc :: new (Mutex :: new (Vec :: new ())) ; if ! nocapture { io :: set_output_capture (Some (data . clone ())) ; } let result = catch_unwind (AssertUnwindSafe (| | bs . bench (f))) ; io :: set_output_capture (None) ; let test_result = match result { Ok (Ok (Some (ns_iter_summ))) => { let ns_iter = cmp :: max (ns_iter_summ . median as u64 , 1) ; let mb_s = bs . bytes * 1000 / ns_iter ; let bs = BenchSamples { ns_iter_summ , mb_s : mb_s as usize } ; TestResult :: TrBench (bs) } Ok (Ok (None)) => { let samples : & mut [f64] = & mut [0.0_f64 ; 1] ; let bs = BenchSamples { ns_iter_summ : stats :: Summary :: new (samples) , mb_s : 0 } ; TestResult :: TrBench (bs) } Err (_) => TestResult :: TrFailed , Ok (Err (_)) => TestResult :: TrFailed , } ; let stdout = data . lock () . unwrap () . to_vec () ; let message = CompletedTest :: new (id , desc , test_result , None , stdout) ; monitor_ch . send (message) . unwrap () ; }
};
}
