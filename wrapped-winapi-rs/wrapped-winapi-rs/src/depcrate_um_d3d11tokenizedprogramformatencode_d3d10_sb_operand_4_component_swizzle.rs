// Generated macro for ENCODE_D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE (function)
macro_rules! Depcrate_um_d3d11tokenizedprogramformatENCODE_D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE {
() => {
// Module: crate::um::d3d11tokenizedprogramformat
// Provides: {"ENCODE_D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE"}
// Dependencies: {}
# [inline] pub fn ENCODE_D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE (XSrc : DWORD , YSrc : DWORD , ZSrc : DWORD , WSrc : DWORD ,) -> DWORD { ((XSrc & D3D10_SB_4_COMPONENT_NAME_MASK) | ((YSrc & D3D10_SB_4_COMPONENT_NAME_MASK) << 2) | ((ZSrc & D3D10_SB_4_COMPONENT_NAME_MASK) << 4) | ((WSrc & D3D10_SB_4_COMPONENT_NAME_MASK) << 6)) << D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE_SHIFT }
};
}
