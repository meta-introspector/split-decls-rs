// Generated macro for ArgExtension (enum)
macro_rules! Depcrate_callconvArgExtension {
() => {
// Module: crate::callconv
// Provides: {"ArgExtension"}
// Dependencies: {}
# [doc = " Sometimes an ABI requires small integers to be extended to a full or partial register. This enum"] # [doc = " defines if this extension should be zero-extension or sign-extension when necessary. When it is"] # [doc = " not necessary to extend the argument, this enum is ignored."] # [derive (Copy , Clone , PartialEq , Eq , Hash , Debug , HashStable_Generic)] pub enum ArgExtension { None , Zext , Sext , }
};
}
