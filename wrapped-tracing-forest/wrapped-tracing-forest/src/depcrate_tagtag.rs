// Generated macro for Tag (struct)
macro_rules! Depcrate_tagTag {
() => {
// Module: crate::tag
// Provides: {"Tag"}
// Dependencies: {}
# [doc = " A basic `Copy` type containing information about where an event occurred."] # [doc = ""] # [doc = " See the [module-level documentation](mod@crate::tag) for more details."] # [derive (Debug , Clone , Copy , Hash , PartialEq , Eq)] pub struct Tag { # [doc = " Optional prefix for the tag message"] prefix : Option < & 'static str > , # [doc = " Level specifying the importance of the log."] # [doc = ""] # [doc = " This value isn't necessarily \"trace\", \"debug\", \"info\", \"warn\", or \"error\","] # [doc = " and can be customized."] suffix : & 'static str , # [doc = " An icon, typically emoji, that represents the tag."] icon : char , }
};
}
