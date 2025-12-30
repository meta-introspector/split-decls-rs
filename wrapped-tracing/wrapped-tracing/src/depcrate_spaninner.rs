// Generated macro for Inner (struct)
macro_rules! Depcrate_spanInner {
() => {
// Module: crate::span
// Provides: {"Inner"}
// Dependencies: {}
# [doc = " A handle representing the capacity to enter a span which is known to exist."] # [doc = ""] # [doc = " Unlike `Span`, this type is only constructed for spans which _have_ been"] # [doc = " enabled by the current filter. This type is primarily used for implementing"] # [doc = " span handles; users should typically not need to interact with it directly."] # [derive (Debug)] pub (crate) struct Inner { # [doc = " The span's ID, as provided by `subscriber`."] id : Id , # [doc = " The subscriber that will receive events relating to this span."] # [doc = ""] # [doc = " This should be the same subscriber that provided this span with its"] # [doc = " `id`."] subscriber : Dispatch , }
};
}
