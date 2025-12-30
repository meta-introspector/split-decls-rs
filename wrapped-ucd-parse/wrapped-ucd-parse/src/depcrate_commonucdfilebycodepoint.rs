// Generated macro for UcdFileByCodepoint (trait)
macro_rules! Depcrate_commonUcdFileByCodepoint {
() => {
// Module: crate::common
// Provides: {"UcdFileByCodepoint"}
// Dependencies: {}
# [doc = " Describes a single UCD file where every record in the file is associated"] # [doc = " with one or more codepoints."] pub trait UcdFileByCodepoint : UcdFile { # [doc = " Returns the codepoints associated with this record."] fn codepoints (& self) -> CodepointIter ; }
};
}
