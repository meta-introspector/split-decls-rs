// Generated macro for Alignment (enum)
macro_rules! DepcrateAlignment {
() => {
// Module: crate
// Provides: {"Alignment"}
// Dependencies: {}
# [doc = " Enum of alignments which are supported."] # [derive (Copy , Clone , Debug , PartialEq , Default)] pub enum Alignment { # [doc = " The value will be aligned to the left."] AlignLeft , # [doc = " The value will be aligned to the right."] AlignRight , # [doc = " The value will be aligned in the center."] AlignCenter , # [doc = " The value will take on a default alignment."] # [default] AlignUnknown , }
};
}
