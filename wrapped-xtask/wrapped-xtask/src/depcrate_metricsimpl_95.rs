// Generated macro for impl_95 (impl)
macro_rules! Depcrate_metricsimpl_95 {
() => {
// Module: crate::metrics
// Provides: {"impl_95"}
// Dependencies: {}
impl Metrics { fn new (sh : & Shell) -> anyhow :: Result < Metrics > { let host = Host :: new (sh) ? ; let timestamp = SystemTime :: now () ; let revision = cmd ! (sh , "git rev-parse HEAD") . read () ? ; let perf_revision = "a584462e145a0c04760fd9391daefb4f6bd13a99" . into () ; Ok (Metrics { host , timestamp , revision , perf_revision , metrics : BTreeMap :: new () }) } fn report (& mut self , name : & str , value : u64 , unit : Unit) { self . metrics . insert (name . into () , (value , unit)) ; } fn json (& self) -> String { let mut buf = String :: new () ; self . to_json (write_json :: object (& mut buf)) ; buf } fn to_json (& self , mut obj : write_json :: Object < '_ >) { self . host . to_json (obj . object ("host")) ; let timestamp = self . timestamp . duration_since (UNIX_EPOCH) . unwrap () ; obj . number ("timestamp" , timestamp . as_secs () as f64) ; obj . string ("revision" , & self . revision) ; obj . string ("perf_revision" , & self . perf_revision) ; let mut metrics = obj . object ("metrics") ; for (k , (value , unit)) in & self . metrics { metrics . array (k) . number (* value as f64) . string (unit) ; } } }
};
}
