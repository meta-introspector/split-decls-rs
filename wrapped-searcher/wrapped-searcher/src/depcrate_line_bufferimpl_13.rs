// Generated macro for impl_13 (impl)
macro_rules! Depcrate_line_bufferimpl_13 {
() => {
// Module: crate::line_buffer
// Provides: {"impl_13"}
// Dependencies: {}
impl BinaryDetection { # [doc = " Returns true if and only if the detection heuristic demands that"] # [doc = " the line buffer stop read data once binary data is observed."] fn is_quit (& self) -> bool { match * self { BinaryDetection :: Quit (_) => true , _ => false , } } }
};
}
