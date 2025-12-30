// Generated macro for CursorShape (enum)
macro_rules! Depcrate_ansiCursorShape {
() => {
// Module: crate::ansi
// Provides: {"CursorShape"}
// Dependencies: {}
# [doc = " Terminal cursor shape."] # [derive (Debug , Default , Eq , PartialEq , Copy , Clone , Hash)] pub enum CursorShape { # [doc = " Cursor is a block like `▒`."] # [default] Block , # [doc = " Cursor is an underscore like `_`."] Underline , # [doc = " Cursor is a vertical bar `⎸`."] Beam , # [doc = " Cursor is a box like `☐`."] HollowBlock , # [doc = " Invisible cursor."] Hidden , }
};
}
