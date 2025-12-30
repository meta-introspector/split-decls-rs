// Generated macro for DECODE_D3D10_SB_OPERAND_TYPE (function)
macro_rules! Depcrate_um_d3d11tokenizedprogramformatDECODE_D3D10_SB_OPERAND_TYPE {
() => {
// Module: crate::um::d3d11tokenizedprogramformat
// Provides: {"DECODE_D3D10_SB_OPERAND_TYPE"}
// Dependencies: {}
# [inline] pub fn DECODE_D3D10_SB_OPERAND_TYPE (OperandToken0 : DWORD) -> D3D10_SB_OPERAND_TYPE { ((OperandToken0 & D3D10_SB_OPERAND_TYPE_MASK) >> D3D10_SB_OPERAND_TYPE_SHIFT) as D3D10_SB_OPERAND_TYPE }
};
}
