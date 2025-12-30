// Generated macro for Chomping (enum)
macro_rules! Depcrate_scannerChomping {
() => {
// Module: crate::scanner
// Provides: {"Chomping"}
// Dependencies: {}
# [doc = " Chomping, how final line breaks and trailing empty lines are interpreted."] # [doc = ""] # [doc = " See YAML spec 8.1.1.2."] # [derive (PartialEq , Eq)] pub enum Chomping { # [doc = " The final line break and any trailing empty lines are excluded."] Strip , # [doc = " The final line break is preserved, but trailing empty lines are excluded."] Clip , # [doc = " The final line break and trailing empty lines are included."] Keep , }
};
}
