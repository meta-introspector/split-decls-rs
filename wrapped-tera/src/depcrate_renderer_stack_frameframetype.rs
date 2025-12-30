// Generated macro for FrameType (enum)
macro_rules! Depcrate_renderer_stack_frameFrameType {
() => {
// Module: crate::renderer::stack_frame
// Provides: {"FrameType"}
// Dependencies: {}
# [doc = " Enumerates the types of stack frames"] # [derive (Clone , Copy , Debug , PartialEq)] pub enum FrameType { # [doc = " Original frame"] Origin , # [doc = " New frame for macro call"] Macro , # [doc = " New frame for for loop"] ForLoop , # [doc = " Include template"] Include , }
};
}
