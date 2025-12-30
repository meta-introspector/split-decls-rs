// Generated macro for impl_723 (impl)
macro_rules! Depcrate_specimpl_723 {
() => {
// Module: crate::spec
// Provides: {"impl_723"}
// Dependencies: {}
impl LinkSelfContainedDefault { # [doc = " Returns whether the target spec has self-contained linking explicitly disabled. Used to emit"] # [doc = " errors if the user then enables it on the CLI."] pub fn is_disabled (self) -> bool { self == LinkSelfContainedDefault :: False } # [doc = " Returns the key to use when serializing the setting to json:"] # [doc = " - individual components in a `link-self-contained` object value"] # [doc = " - the other variants as a backwards-compatible `crt-objects-fallback` string"] fn json_key (self) -> & 'static str { match self { LinkSelfContainedDefault :: WithComponents (_) => "link-self-contained" , _ => "crt-objects-fallback" , } } # [doc = " Creates a `LinkSelfContainedDefault` enabling the self-contained linker for target specs"] # [doc = " (the equivalent of `-Clink-self-contained=+linker` on the CLI)."] pub fn with_linker () -> LinkSelfContainedDefault { LinkSelfContainedDefault :: WithComponents (LinkSelfContainedComponents :: LINKER) } }
};
}
