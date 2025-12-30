// Generated macro for Color (enum)
macro_rules! Depcrate_ansiColor {
() => {
// Module: crate::ansi
// Provides: {"Color"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub enum Color { Named (NamedColor) , Spec (Rgb) , Indexed (u8) , }
};
}
