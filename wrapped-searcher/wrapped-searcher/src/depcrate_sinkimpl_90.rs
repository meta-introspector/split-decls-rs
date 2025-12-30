// Generated macro for impl_90 (impl)
macro_rules! Depcrate_sinkimpl_90 {
() => {
// Module: crate::sink
// Provides: {"impl_90"}
// Dependencies: {}
# [doc = " An `std::io::Error` can be used as an error for `Sink` implementations out"] # [doc = " of the box."] impl SinkError for io :: Error { fn error_message < T : std :: fmt :: Display > (message : T) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , message . to_string ()) } fn error_io (err : io :: Error) -> io :: Error { err } }
};
}
