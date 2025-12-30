// Generated macro for is_feature_flaggable (function)
macro_rules! Depcrateis_feature_flaggable {
() => {
// Module: crate
// Provides: {"is_feature_flaggable"}
// Dependencies: {}
# [doc = " Checks whether the running or installed `rustc` supports feature flags."] # [doc = ""] # [doc = " Returns true if the channel is either \"nightly\" or \"dev\"."] # [doc = ""] # [doc = " **Please see the note on [feature detection](crate#feature-detection).**"] # [doc = ""] # [doc = " Note that support for specific `rustc` features can be enabled or disabled"] # [doc = " via the `allow-features` compiler flag, which this function _does not_"] # [doc = " check. That is, this function _does not_ check whether a _specific_ feature"] # [doc = " is supported, but instead whether features are supported at all. To check"] # [doc = " for support for a specific feature, use [`supports_feature()`]."] # [doc = ""] # [doc = " If the version could not be determined, returns `None`. Otherwise returns"] # [doc = " `true` if the running version supports feature flags and `false` otherwise."] pub fn is_feature_flaggable () -> Option < bool > { Channel :: read () . map (| c | c . supports_features ()) }
};
}
