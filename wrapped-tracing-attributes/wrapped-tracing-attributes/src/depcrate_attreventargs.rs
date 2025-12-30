// Generated macro for EventArgs (struct)
macro_rules! Depcrate_attrEventArgs {
() => {
// Module: crate::attr
// Provides: {"EventArgs"}
// Dependencies: {}
# [doc = " Arguments to `#[instrument(err(...))]` and `#[instrument(ret(...))]` which describe how the"] # [doc = " return value event should be emitted."] # [derive (Clone , Default , Debug)] pub (crate) struct EventArgs { level : Option < Level > , pub (crate) mode : FormatMode , }
};
}
