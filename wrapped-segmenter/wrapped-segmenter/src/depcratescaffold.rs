// Generated macro for scaffold (module)
macro_rules! Depcratescaffold {
() => {
// Module: crate
// Provides: {"scaffold"}
// Dependencies: {}
# [doc = " Largely-internal scaffolding types (You should very rarely need to reference these directly)"] pub mod scaffold { pub use crate :: line :: LineBreakType ; pub use crate :: rule_segmenter :: { Latin1 , PotentiallyIllFormedUtf8 , RuleBreakType , Utf16 , Utf8 } ; pub use crate :: word :: WordBreakType ; }
};
}
