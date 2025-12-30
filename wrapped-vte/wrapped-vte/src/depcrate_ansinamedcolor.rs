// Generated macro for NamedColor (enum)
macro_rules! Depcrate_ansiNamedColor {
() => {
// Module: crate::ansi
// Provides: {"NamedColor"}
// Dependencies: {}
# [doc = " Standard colors."] # [doc = ""] # [doc = " The order here matters since the enum should be castable to a `usize` for"] # [doc = " indexing a color list."] # [derive (Debug , Copy , Clone , Eq , PartialEq , PartialOrd , Ord)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub enum NamedColor { # [doc = " Black."] Black = 0 , # [doc = " Red."] Red , # [doc = " Green."] Green , # [doc = " Yellow."] Yellow , # [doc = " Blue."] Blue , # [doc = " Magenta."] Magenta , # [doc = " Cyan."] Cyan , # [doc = " White."] White , # [doc = " Bright black."] BrightBlack , # [doc = " Bright red."] BrightRed , # [doc = " Bright green."] BrightGreen , # [doc = " Bright yellow."] BrightYellow , # [doc = " Bright blue."] BrightBlue , # [doc = " Bright magenta."] BrightMagenta , # [doc = " Bright cyan."] BrightCyan , # [doc = " Bright white."] BrightWhite , # [doc = " The foreground color."] Foreground = 256 , # [doc = " The background color."] Background , # [doc = " Color for the cursor itself."] Cursor , # [doc = " Dim black."] DimBlack , # [doc = " Dim red."] DimRed , # [doc = " Dim green."] DimGreen , # [doc = " Dim yellow."] DimYellow , # [doc = " Dim blue."] DimBlue , # [doc = " Dim magenta."] DimMagenta , # [doc = " Dim cyan."] DimCyan , # [doc = " Dim white."] DimWhite , # [doc = " The bright foreground color."] BrightForeground , # [doc = " Dim foreground."] DimForeground , }
};
}
