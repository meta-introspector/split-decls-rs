// Generated macro for AsciiProbeResult (struct)
macro_rules! Depcrate_cursorAsciiProbeResult {
() => {
// Module: crate::cursor
// Provides: {"AsciiProbeResult"}
// Dependencies: {}
# [doc = " Information about a probed edge."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [non_exhaustive] pub struct AsciiProbeResult { # [doc = " The character's byte value between this node and its parent."] pub byte : u8 , # [doc = " The number of siblings of this node, _including itself_."] pub total_siblings : u8 , }
};
}
