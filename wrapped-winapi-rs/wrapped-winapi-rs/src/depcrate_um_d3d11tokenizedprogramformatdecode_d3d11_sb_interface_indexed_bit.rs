// Generated macro for DECODE_D3D11_SB_INTERFACE_INDEXED_BIT (function)
macro_rules! Depcrate_um_d3d11tokenizedprogramformatDECODE_D3D11_SB_INTERFACE_INDEXED_BIT {
() => {
// Module: crate::um::d3d11tokenizedprogramformat
// Provides: {"DECODE_D3D11_SB_INTERFACE_INDEXED_BIT"}
// Dependencies: {}
# [inline] pub fn DECODE_D3D11_SB_INTERFACE_INDEXED_BIT (OpcodeToken0 : DWORD) -> DWORD { if (OpcodeToken0 & D3D11_SB_INTERFACE_INDEXED_BIT_MASK) >> D3D11_SB_INTERFACE_INDEXED_BIT_SHIFT != 0 { 1 } else { 0 } }
};
}
