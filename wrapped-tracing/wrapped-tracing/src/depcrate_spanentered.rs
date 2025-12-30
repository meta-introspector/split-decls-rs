// Generated macro for Entered (struct)
macro_rules! Depcrate_spanEntered {
() => {
// Module: crate::span
// Provides: {"Entered"}
// Dependencies: {}
# [doc = " A guard representing a span which has been entered and is currently"] # [doc = " executing."] # [doc = ""] # [doc = " When the guard is dropped, the span will be exited."] # [doc = ""] # [doc = " This is returned by the [`Span::enter`] function."] # [doc = ""] # [doc = " [`Span::enter`]: super::Span::enter"] # [derive (Debug)] # [must_use = "once a span has been entered, it should be exited"] pub struct Entered < 'a > { span : & 'a Span , }
};
}
