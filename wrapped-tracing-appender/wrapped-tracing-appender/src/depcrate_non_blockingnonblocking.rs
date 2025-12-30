// Generated macro for NonBlocking (struct)
macro_rules! Depcrate_non_blockingNonBlocking {
() => {
// Module: crate::non_blocking
// Provides: {"NonBlocking"}
// Dependencies: {}
# [doc = " A non-blocking writer."] # [doc = ""] # [doc = " While the line between \"blocking\" and \"non-blocking\" IO is fuzzy, writing to a file is typically"] # [doc = " considered to be a _blocking_ operation. For an application whose `Subscriber` writes spans and events"] # [doc = " as they are emitted, an application might find the latency profile to be unacceptable."] # [doc = " `NonBlocking` moves the writing out of an application's data path by sending spans and events"] # [doc = " to a dedicated logging thread."] # [doc = ""] # [doc = " This struct implements [`MakeWriter`] from the `tracing-subscriber`"] # [doc = " crate. Therefore, it can be used with the [`tracing_subscriber::fmt`][fmt] module"] # [doc = " or with any other subscriber/layer implementation that uses the `MakeWriter` trait."] # [doc = ""] # [doc = " [fmt]: mod@tracing_subscriber::fmt"] # [derive (Clone , Debug)] pub struct NonBlocking { error_counter : ErrorCounter , channel : Sender < Msg > , is_lossy : bool , }
};
}
