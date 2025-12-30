// Generated macro for Performer (struct)
macro_rules! Depcrate_ansiPerformer {
() => {
// Module: crate::ansi
// Provides: {"Performer"}
// Dependencies: {}
# [doc = " Helper type that implements `crate::Perform`."] # [doc = ""] # [doc = " Processor creates a Performer when running advance and passes the Performer"] # [doc = " to `crate::Parser`."] struct Performer < 'a , H : Handler , T : Timeout > { state : & 'a mut ProcessorState < T > , handler : & 'a mut H , # [doc = " Whether the parser should be prematurely terminated."] terminated : bool , }
};
}
