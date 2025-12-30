// Generated macro for ModeType (enum)
macro_rules! Depcrate_blocks_sequence_sectionModeType {
() => {
// Module: crate::blocks::sequence_section
// Provides: {"ModeType"}
// Dependencies: {}
# [doc = " The compression mode used for symbol compression"] pub enum ModeType { # [doc = " A predefined FSE distribution table is used, and no distribution table"] # [doc = " will be present."] Predefined , # [doc = " The table consists of a single byte, which contains the symbol's value."] # [allow (clippy :: upper_case_acronyms)] RLE , # [doc = " Standard FSE compression, a distribution table will be present. This"] # [doc = " mode should not be used when only one symbol is present."] FSECompressed , # [doc = " The table used in the previous compressed block with at least one sequence"] # [doc = " will be used again. If this is the first block, the table in the dictionary will"] # [doc = " be used."] Repeat , }
};
}
