// Generated macro for EnteredSpan (struct)
macro_rules! Depcrate_spanEnteredSpan {
() => {
// Module: crate::span
// Provides: {"EnteredSpan"}
// Dependencies: {}
# [doc = " An owned version of [`Entered`], a guard representing a span which has been"] # [doc = " entered and is currently executing."] # [doc = ""] # [doc = " When the guard is dropped, the span will be exited."] # [doc = ""] # [doc = " This is returned by the [`Span::entered`] function."] # [doc = ""] # [doc = " [`Span::entered`]: super::Span::entered()"] # [derive (Debug)] # [must_use = "once a span has been entered, it should be exited"] pub struct EnteredSpan { span : Span , # [doc = " ```compile_fail"] # [doc = " use tracing::span::*;"] # [doc = " trait AssertSend: Send {}"] # [doc = ""] # [doc = " impl AssertSend for EnteredSpan {}"] # [doc = " ```"] _not_send : PhantomNotSend , }
};
}
