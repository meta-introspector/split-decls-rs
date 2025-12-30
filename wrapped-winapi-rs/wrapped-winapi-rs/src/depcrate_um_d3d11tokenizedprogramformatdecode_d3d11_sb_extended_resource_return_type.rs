// Generated macro for DECODE_D3D11_SB_EXTENDED_RESOURCE_RETURN_TYPE (function)
macro_rules! Depcrate_um_d3d11tokenizedprogramformatDECODE_D3D11_SB_EXTENDED_RESOURCE_RETURN_TYPE {
() => {
// Module: crate::um::d3d11tokenizedprogramformat
// Provides: {"DECODE_D3D11_SB_EXTENDED_RESOURCE_RETURN_TYPE"}
// Dependencies: {}
# [inline] pub fn DECODE_D3D11_SB_EXTENDED_RESOURCE_RETURN_TYPE (OpcodeTokenN : DWORD , Component : DWORD ,) -> DWORD { ((OpcodeTokenN >> (Component * D3D10_SB_RESOURCE_RETURN_TYPE_NUMBITS + D3D11_SB_EXTENDED_RESOURCE_RETURN_TYPE_SHIFT)) & D3D10_SB_RESOURCE_RETURN_TYPE_MASK) as D3D10_SB_RESOURCE_RETURN_TYPE }
};
}
