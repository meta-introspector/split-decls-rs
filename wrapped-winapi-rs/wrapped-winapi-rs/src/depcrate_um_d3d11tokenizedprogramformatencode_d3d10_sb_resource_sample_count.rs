// Generated macro for ENCODE_D3D10_SB_RESOURCE_SAMPLE_COUNT (function)
macro_rules! Depcrate_um_d3d11tokenizedprogramformatENCODE_D3D10_SB_RESOURCE_SAMPLE_COUNT {
() => {
// Module: crate::um::d3d11tokenizedprogramformat
// Provides: {"ENCODE_D3D10_SB_RESOURCE_SAMPLE_COUNT"}
// Dependencies: {}
# [inline] pub fn ENCODE_D3D10_SB_RESOURCE_SAMPLE_COUNT (SampleCount : DWORD) -> DWORD { (if SampleCount > 127 { 127 } else { SampleCount } << D3D10_SB_RESOURCE_SAMPLE_COUNT_SHIFT) & D3D10_SB_RESOURCE_SAMPLE_COUNT_MASK }
};
}
