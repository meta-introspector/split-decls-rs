// Generated macro for Color (enum)
macro_rules! DepcrateColor {
() => {
// Module: crate
// Provides: {"Color"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq , EnumString , Display , EnumCount , EnumDiscriminants , EnumIs)] pub enum Color { # [doc = " Docs on red"] # [strum (to_string = "RedRed")] Red , # [strum (serialize = "b" , to_string = "blue")] Blue { hue : usize } , # [strum (serialize = "y" , serialize = "yellow")] Yellow , # [strum (disabled)] Green (String) , }
};
}
