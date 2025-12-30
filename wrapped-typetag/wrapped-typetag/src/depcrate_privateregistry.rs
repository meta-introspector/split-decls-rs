// Generated macro for Registry (struct)
macro_rules! Depcrate_privateRegistry {
() => {
// Module: crate::private
// Provides: {"Registry"}
// Dependencies: {}
# [doc (hidden)] pub struct Registry < T : ? Sized > { # [doc (hidden)] pub map : BTreeMap < & 'static str , Option < DeserializeFn < T > > > , # [doc (hidden)] pub names : Vec < & 'static str > , }
};
}
