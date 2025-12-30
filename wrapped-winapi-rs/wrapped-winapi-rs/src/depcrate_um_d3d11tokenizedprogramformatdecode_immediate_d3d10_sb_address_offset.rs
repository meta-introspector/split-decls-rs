// Generated macro for DECODE_IMMEDIATE_D3D10_SB_ADDRESS_OFFSET (function)
macro_rules! Depcrate_um_d3d11tokenizedprogramformatDECODE_IMMEDIATE_D3D10_SB_ADDRESS_OFFSET {
() => {
// Module: crate::um::d3d11tokenizedprogramformat
// Provides: {"DECODE_IMMEDIATE_D3D10_SB_ADDRESS_OFFSET"}
// Dependencies: {}
# [inline] pub fn DECODE_IMMEDIATE_D3D10_SB_ADDRESS_OFFSET (Coord : DWORD , OpcodeToken1 : DWORD ,) -> DWORD { (OpcodeToken1 & D3D10_SB_IMMEDIATE_ADDRESS_OFFSET_MASK (Coord)) >> D3D10_SB_IMMEDIATE_ADDRESS_OFFSET_SHIFT (Coord) }
};
}
