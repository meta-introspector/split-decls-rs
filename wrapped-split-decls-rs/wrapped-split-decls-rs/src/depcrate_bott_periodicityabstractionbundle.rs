// Generated macro for AbstractionBundle (struct)
macro_rules! Depcrate_bott_periodicityAbstractionBundle {
() => {
// Module: crate::bott_periodicity
// Provides: {"AbstractionBundle"}
// Dependencies: {}
# [doc = " An abstraction at level n in the periodic tower"] # [derive (Debug , Clone)] pub struct AbstractionBundle { # [doc = " Which level (mod 8) in the Bott tower"] pub bott_level : BottLevel , # [doc = " Absolute level (how many times we've gone around)"] pub winding_number : usize , # [doc = " The actual content (shape repeats, but \"meaning\" differs)"] pub content : AbstractionContent , # [doc = " Characteristic classes (topological invariants)"] pub chern_classes : Vec < i32 > , }
};
}
