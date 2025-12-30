// Generated macro for WithDispatch (struct)
macro_rules! DepcrateWithDispatch {
() => {
// Module: crate
// Provides: {"WithDispatch"}
// Dependencies: {}
# [doc = " A future, stream, sink, or executor that has been instrumented with a"] # [doc = " `tracing` subscriber."] # [cfg (all (feature = "std" , not (feature = "std-future")))] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] # [derive (Clone , Debug)] pub struct WithDispatch < T > { inner : T , dispatch : Dispatch , }
};
}
