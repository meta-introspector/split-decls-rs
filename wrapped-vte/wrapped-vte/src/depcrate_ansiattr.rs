// Generated macro for Attr (enum)
macro_rules! Depcrate_ansiAttr {
() => {
// Module: crate::ansi
// Provides: {"Attr"}
// Dependencies: {}
# [doc = " Terminal character attributes."] # [derive (Debug , Eq , PartialEq)] pub enum Attr { # [doc = " Clear all special abilities."] Reset , # [doc = " Bold text."] Bold , # [doc = " Dim or secondary color."] Dim , # [doc = " Italic text."] Italic , # [doc = " Underline text."] Underline , # [doc = " Underlined twice."] DoubleUnderline , # [doc = " Undercurled text."] Undercurl , # [doc = " Dotted underlined text."] DottedUnderline , # [doc = " Dashed underlined text."] DashedUnderline , # [doc = " Blink cursor slowly."] BlinkSlow , # [doc = " Blink cursor fast."] BlinkFast , # [doc = " Invert colors."] Reverse , # [doc = " Do not display characters."] Hidden , # [doc = " Strikeout text."] Strike , # [doc = " Cancel bold."] CancelBold , # [doc = " Cancel bold and dim."] CancelBoldDim , # [doc = " Cancel italic."] CancelItalic , # [doc = " Cancel all underlines."] CancelUnderline , # [doc = " Cancel blink."] CancelBlink , # [doc = " Cancel inversion."] CancelReverse , # [doc = " Cancel text hiding."] CancelHidden , # [doc = " Cancel strikeout."] CancelStrike , # [doc = " Set indexed foreground color."] Foreground (Color) , # [doc = " Set indexed background color."] Background (Color) , # [doc = " Underline color."] UnderlineColor (Option < Color >) , }
};
}
