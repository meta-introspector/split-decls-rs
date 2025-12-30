// Generated macro for ENCODE_D3D10_SB_OPERAND_INDEX_REPRESENTATION (function)
macro_rules! Depcrate_um_d3d11tokenizedprogramformatENCODE_D3D10_SB_OPERAND_INDEX_REPRESENTATION {
() => {
// Module: crate::um::d3d11tokenizedprogramformat
// Provides: {"ENCODE_D3D10_SB_OPERAND_INDEX_REPRESENTATION"}
// Dependencies: {}
# [inline] pub fn ENCODE_D3D10_SB_OPERAND_INDEX_REPRESENTATION (Dim : DWORD , IndexRepresentation : D3D10_SB_OPERAND_INDEX_REPRESENTATION ,) -> DWORD { (IndexRepresentation << D3D10_SB_OPERAND_INDEX_REPRESENTATION_SHIFT (Dim)) & D3D10_SB_OPERAND_INDEX_REPRESENTATION_MASK (Dim) }
};
}
