// Generated macro for impl_416 (impl)
macro_rules! Depcrate_stream_recoverableimpl_416 {
() => {
// Module: crate::stream::recoverable
// Provides: {"impl_416"}
// Dependencies: {}
impl < I , E > Recoverable < I , E > where I : Stream , { # [doc = " Track recoverable errors with the stream"] # [inline] pub fn new (input : I) -> Self { Self { input , errors : Default :: default () , is_recoverable : true , } } # [doc = " Act as a normal stream"] # [inline] pub fn unrecoverable (input : I) -> Self { Self { input , errors : Default :: default () , is_recoverable : false , } } # [doc = " Access the current input and errors"] # [inline] pub fn into_parts (self) -> (I , Vec < E >) { (self . input , self . errors) } }
};
}
