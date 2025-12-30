// Generated macro for prelude (module)
macro_rules! Depcrateprelude {
() => {
// Module: crate
// Provides: {"prelude"}
// Dependencies: {}
# [cfg (feature = "traced-error")] # [cfg_attr (docsrs , doc (cfg (feature = "traced-error")))] pub mod prelude { # ! [doc = " The `tracing-error` prelude."] # ! [doc = ""] # ! [doc = " This brings into scope the `InstrumentError`, `InstrumentResult`, and `ExtractSpanTrace`"] # ! [doc = " extension traits. These traits allow attaching `SpanTrace`s to errors and"] # ! [doc = " subsequently retrieving them from `dyn Error` trait objects."] # ! [allow (unreachable_pub)] pub use crate :: { ExtractSpanTrace as _ , InstrumentError as _ , InstrumentResult as _ } ; }
};
}
