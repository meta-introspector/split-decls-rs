// Generated macro for Current (struct)
macro_rules! Depcrate_spanCurrent {
() => {
// Module: crate::span
// Provides: {"Current"}
// Dependencies: {}
# [doc = " Indicates what [the `Subscriber` considers] the \"current\" span."] # [doc = ""] # [doc = " As subscribers may not track a notion of a current span, this has three"] # [doc = " possible states:"] # [doc = " - \"unknown\", indicating that the subscriber does not track a current span,"] # [doc = " - \"none\", indicating that the current context is known to not be in a span,"] # [doc = " - \"some\", with the current span's [`Id`] and [`Metadata`]."] # [doc = ""] # [doc = " [the `Subscriber` considers]: super::subscriber::Subscriber::current_span"] # [doc = " [`Metadata`]: super::metadata::Metadata"] # [derive (Debug)] pub struct Current { inner : CurrentInner , }
};
}
