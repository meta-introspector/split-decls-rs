// Generated macro for ExternalSource (enum)
macro_rules! DepcrateExternalSource {
() => {
// Module: crate
// Provides: {"ExternalSource"}
// Dependencies: {}
# [derive (PartialEq , Eq , Clone , Debug)] pub enum ExternalSource { # [doc = " No external source has to be loaded, since the `SourceFile` represents a local crate."] Unneeded , Foreign { kind : ExternalSourceKind , # [doc = " Index of the file inside metadata."] metadata_index : u32 , } , }
};
}
