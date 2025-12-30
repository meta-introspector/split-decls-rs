// Generated macro for impl_585 (impl)
macro_rules! Depcrate_traceimpl_585 {
() => {
// Module: crate::trace
// Provides: {"impl_585"}
// Dependencies: {}
impl fmt :: Display for Latency { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . unit { LatencyUnit :: Seconds => write ! (f , "{} s" , self . duration . as_secs_f64 ()) , LatencyUnit :: Millis => write ! (f , "{} ms" , self . duration . as_millis ()) , LatencyUnit :: Micros => write ! (f , "{} μs" , self . duration . as_micros ()) , LatencyUnit :: Nanos => write ! (f , "{} ns" , self . duration . as_nanos ()) , } } }
};
}
