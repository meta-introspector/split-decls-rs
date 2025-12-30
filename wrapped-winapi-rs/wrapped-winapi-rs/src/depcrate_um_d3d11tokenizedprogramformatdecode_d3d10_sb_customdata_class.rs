// Generated macro for DECODE_D3D10_SB_CUSTOMDATA_CLASS (function)
macro_rules! Depcrate_um_d3d11tokenizedprogramformatDECODE_D3D10_SB_CUSTOMDATA_CLASS {
() => {
// Module: crate::um::d3d11tokenizedprogramformat
// Provides: {"DECODE_D3D10_SB_CUSTOMDATA_CLASS"}
// Dependencies: {}
# [inline] pub fn DECODE_D3D10_SB_CUSTOMDATA_CLASS (CustomDataDescTok : DWORD) -> D3D10_SB_CUSTOMDATA_CLASS { ((CustomDataDescTok & D3D10_SB_CUSTOMDATA_CLASS_MASK) >> D3D10_SB_CUSTOMDATA_CLASS_SHIFT) as D3D10_SB_CUSTOMDATA_CLASS }
};
}
