// Generated macro for ENCODE_D3D11_SB_EXTENDED_RESOURCE_RETURN_TYPE (function)
macro_rules! Depcrate_um_d3d11tokenizedprogramformatENCODE_D3D11_SB_EXTENDED_RESOURCE_RETURN_TYPE {
() => {
// Module: crate::um::d3d11tokenizedprogramformat
// Provides: {"ENCODE_D3D11_SB_EXTENDED_RESOURCE_RETURN_TYPE"}
// Dependencies: {}
# [inline] pub fn ENCODE_D3D11_SB_EXTENDED_RESOURCE_RETURN_TYPE (ReturnType : DWORD , Component : DWORD ,) -> DWORD { (ReturnType & D3D10_SB_RESOURCE_RETURN_TYPE_MASK) << (Component * D3D10_SB_RESOURCE_RETURN_TYPE_NUMBITS + D3D11_SB_EXTENDED_RESOURCE_RETURN_TYPE_SHIFT) }
};
}
