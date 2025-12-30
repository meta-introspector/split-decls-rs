// Generated macro for Processor (struct)
macro_rules! Depcrate_ansiProcessor {
() => {
// Module: crate::ansi
// Provides: {"Processor"}
// Dependencies: {}
# [doc = " The processor wraps a `crate::Parser` to ultimately call methods on a"] # [doc = " Handler."] # [cfg (not (feature = "std"))] # [derive (Default)] pub struct Processor < T : Timeout > { state : ProcessorState < T > , parser : crate :: Parser , }
};
}
