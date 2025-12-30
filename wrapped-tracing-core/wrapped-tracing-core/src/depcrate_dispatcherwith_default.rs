// Generated macro for with_default (function)
macro_rules! Depcrate_dispatcherwith_default {
() => {
// Module: crate::dispatcher
// Provides: {"with_default"}
// Dependencies: {}
# [doc = " Sets this dispatch as the default for the duration of a closure."] # [doc = ""] # [doc = " The default dispatcher is used when creating a new [span] or"] # [doc = " [`Event`]."] # [doc = ""] # [doc = " <pre class=\"ignore\" style=\"white-space:normal;font:inherit;\">"] # [doc = "     <strong>Note</strong>: This function required the Rust standard library."] # [doc = "     <code>no_std</code> users should use <a href=\"fn.set_global_default.html\">"] # [doc = "     <code>set_global_default</code></a> instead."] # [doc = " </pre>"] # [doc = ""] # [doc = " [span]: super::span"] # [doc = " [`Subscriber`]: super::subscriber::Subscriber"] # [doc = " [`Event`]: super::event::Event"] # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub fn with_default < T > (dispatcher : & Dispatch , f : impl FnOnce () -> T) -> T { let _guard = set_default (dispatcher) ; f () }
};
}
