// Generated macro for SinkError (trait)
macro_rules! Depcrate_sinkSinkError {
() => {
// Module: crate::sink
// Provides: {"SinkError"}
// Dependencies: {}
# [doc = " A trait that describes errors that can be reported by searchers and"] # [doc = " implementations of `Sink`."] # [doc = ""] # [doc = " Unless you have a specialized use case, you probably don't need to"] # [doc = " implement this trait explicitly. It's likely that using `std::io::Error`"] # [doc = " (which implements this trait) for your error type is good enough,"] # [doc = " largely because most errors that occur during search will likely be an"] # [doc = " `std::io::Error`."] pub trait SinkError : Sized { # [doc = " A constructor for converting any value that satisfies the"] # [doc = " `std::fmt::Display` trait into an error."] fn error_message < T : std :: fmt :: Display > (message : T) -> Self ; # [doc = " A constructor for converting I/O errors that occur while searching into"] # [doc = " an error of this type."] # [doc = ""] # [doc = " By default, this is implemented via the `error_message` constructor."] fn error_io (err : io :: Error) -> Self { Self :: error_message (err) } # [doc = " A constructor for converting configuration errors that occur while"] # [doc = " building a searcher into an error of this type."] # [doc = ""] # [doc = " By default, this is implemented via the `error_message` constructor."] fn error_config (err : ConfigError) -> Self { Self :: error_message (err) } }
};
}
