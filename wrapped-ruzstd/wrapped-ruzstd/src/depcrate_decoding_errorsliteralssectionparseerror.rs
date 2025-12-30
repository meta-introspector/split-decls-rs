// Generated macro for LiteralsSectionParseError (enum)
macro_rules! Depcrate_decoding_errorsLiteralsSectionParseError {
() => {
// Module: crate::decoding::errors
// Provides: {"LiteralsSectionParseError"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum LiteralsSectionParseError { IllegalLiteralSectionType { got : u8 } , GetBitsError (GetBitsError) , NotEnoughBytes { have : usize , need : u8 } , }
};
}
