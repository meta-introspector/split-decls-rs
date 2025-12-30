// Generated macro for ENCODE_D3D10_SB_CUSTOMDATA_CLASS (function)
macro_rules! Depcrate_um_d3d11tokenizedprogramformatENCODE_D3D10_SB_CUSTOMDATA_CLASS {
() => {
// Module: crate::um::d3d11tokenizedprogramformat
// Provides: {"ENCODE_D3D10_SB_CUSTOMDATA_CLASS"}
// Dependencies: {}
# [inline] pub fn ENCODE_D3D10_SB_CUSTOMDATA_CLASS (CustomDataClass : D3D10_SB_CUSTOMDATA_CLASS) -> DWORD { ENCODE_D3D10_SB_OPCODE_TYPE (D3D10_SB_OPCODE_CUSTOMDATA) | ((CustomDataClass << D3D10_SB_CUSTOMDATA_CLASS_SHIFT) & D3D10_SB_CUSTOMDATA_CLASS_MASK) }
};
}
