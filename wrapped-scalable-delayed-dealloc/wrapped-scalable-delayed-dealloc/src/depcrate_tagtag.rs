// Generated macro for Tag (enum)
macro_rules! Depcrate_tagTag {
() => {
// Module: crate::tag
// Provides: {"Tag"}
// Dependencies: {}
# [doc = " [`Tag`] is a four-state `Enum` that can be embedded in a pointer as the two least"] # [doc = " significant bits of the pointer value."] # [derive (Clone , Copy , Debug , Eq , Ord , PartialEq , PartialOrd)] pub enum Tag { # [doc = " None tagged."] None , # [doc = " The first bit is tagged."] First , # [doc = " The second bit is tagged."] Second , # [doc = " Both bits are tagged."] Both , }
};
}
