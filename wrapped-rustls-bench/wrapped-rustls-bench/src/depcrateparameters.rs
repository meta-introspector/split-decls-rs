// Generated macro for Parameters (struct)
macro_rules! DepcrateParameters {
() => {
// Module: crate
// Provides: {"Parameters"}
// Dependencies: {}
# [doc = " General parameters common to several kinds of benchmark."] # [derive (Clone)] struct Parameters { # [doc = " Set by the user."] work_multiplier : f64 , latency_prefix : Option < String > , provider : Provider , api : Api , threads : NonZeroUsize , # [doc = " A compatible key/cipher suite/version combination."] proto : BenchmarkParam , # [doc = " Whether the client authenticates."] client_auth : ClientAuth , # [doc = " Whether the sessions are resumed."] resume : ResumptionParam , # [doc = " The maximum fragment size (if any)."] max_fragment_size : Option < usize > , # [doc = " For bulk benchmarks, how much data to send"] plaintext_size : u64 , }
};
}
