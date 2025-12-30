// Generated macro for CodepointRange (struct)
macro_rules! Depcrate_unicode_dataCodepointRange {
() => {
// Module: crate::unicode_data
// Provides: {"CodepointRange"}
// Dependencies: {}
struct CodepointRange { # [doc = " The codepoint range."] range : std :: ops :: Range < u32 > , # [doc = " The start record. All subsequent records in this range are generated"] # [doc = " by cloning this and updating the codepoint/name."] start_record : UnicodeData , }
};
}
