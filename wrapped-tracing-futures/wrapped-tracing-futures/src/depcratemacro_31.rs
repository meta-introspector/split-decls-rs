// Generated macro for macro_31 (macro)
macro_rules! Depcratemacro_31 {
() => {
// Module: crate
// Provides: {"macro_31"}
// Dependencies: {}
# [cfg (all (feature = "std" , feature = "std-future"))] pin_project ! { # [doc = " A future, stream, sink, or executor that has been instrumented with a"] # [doc = " `tracing` subscriber."] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] # [derive (Clone , Debug)] pub struct WithDispatch < T > { # [pin] inner : T , dispatch : Dispatch , } }
};
}
