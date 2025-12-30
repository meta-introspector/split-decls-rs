// Generated macro for Document (struct)
macro_rules! Depcrate_documentDocument {
() => {
// Module: crate::document
// Provides: {"Document"}
// Dependencies: {}
# [doc = " The root TOML [`Table`], containing [`Key`][crate::Key]/[`Value`][crate::Value] pairs and all other logic [`Table`]s"] # [derive (Debug , Clone)] pub struct Document < S > { pub (crate) root : Item , pub (crate) trailing : RawString , pub (crate) raw : S , }
};
}
