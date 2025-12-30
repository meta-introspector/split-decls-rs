// Generated macro for sys (module)
macro_rules! Depcrate_loom_stdsys {
() => {
// Module: crate::loom::std
// Provides: {"sys"}
// Dependencies: {}
pub (crate) mod sys { # [cfg (feature = "rt-multi-thread")] pub (crate) fn num_cpus () -> usize { use std :: num :: NonZeroUsize ; const ENV_WORKER_THREADS : & str = "TOKIO_WORKER_THREADS" ; match std :: env :: var (ENV_WORKER_THREADS) { Ok (s) => { let n = s . parse () . unwrap_or_else (| e | { panic ! ("\"{ENV_WORKER_THREADS}\" must be usize, error: {e}, value: {s}") }) ; assert ! (n > 0 , "\"{ENV_WORKER_THREADS}\" cannot be set to 0") ; n } Err (std :: env :: VarError :: NotPresent) => { std :: thread :: available_parallelism () . map_or (1 , NonZeroUsize :: get) } Err (std :: env :: VarError :: NotUnicode (e)) => { panic ! ("\"{ENV_WORKER_THREADS}\" must be valid unicode, error: {e:?}") } } } # [cfg (not (feature = "rt-multi-thread"))] pub (crate) fn num_cpus () -> usize { 1 } }
};
}
