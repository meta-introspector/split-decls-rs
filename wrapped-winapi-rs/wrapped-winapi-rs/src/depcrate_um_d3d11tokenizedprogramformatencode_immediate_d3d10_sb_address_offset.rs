// Generated macro for ENCODE_IMMEDIATE_D3D10_SB_ADDRESS_OFFSET (function)
macro_rules! Depcrate_um_d3d11tokenizedprogramformatENCODE_IMMEDIATE_D3D10_SB_ADDRESS_OFFSET {
() => {
// Module: crate::um::d3d11tokenizedprogramformat
// Provides: {"ENCODE_IMMEDIATE_D3D10_SB_ADDRESS_OFFSET"}
// Dependencies: {}
# [inline] pub fn ENCODE_IMMEDIATE_D3D10_SB_ADDRESS_OFFSET (Coord : DWORD , ImmediateOffset : DWORD ,) -> DWORD { (ImmediateOffset << D3D10_SB_IMMEDIATE_ADDRESS_OFFSET_SHIFT (Coord)) & D3D10_SB_IMMEDIATE_ADDRESS_OFFSET_MASK (Coord) }
};
}
