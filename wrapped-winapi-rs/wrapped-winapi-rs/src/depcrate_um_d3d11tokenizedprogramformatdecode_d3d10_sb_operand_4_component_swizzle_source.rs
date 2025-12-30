// Generated macro for DECODE_D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE_SOURCE (function)
macro_rules! Depcrate_um_d3d11tokenizedprogramformatDECODE_D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE_SOURCE {
() => {
// Module: crate::um::d3d11tokenizedprogramformat
// Provides: {"DECODE_D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE_SOURCE"}
// Dependencies: {}
# [inline] pub fn DECODE_D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE_SOURCE (OperandToken0 : DWORD , DestComp : DWORD ,) -> D3D10_SB_4_COMPONENT_NAME { ((OperandToken0 >> (D3D10_SB_OPERAND_4_COMPONENT_SWIZZLE_SHIFT + 2 * (DestComp & D3D10_SB_4_COMPONENT_NAME_MASK))) & D3D10_SB_4_COMPONENT_NAME_MASK) as D3D10_SB_4_COMPONENT_NAME }
};
}
