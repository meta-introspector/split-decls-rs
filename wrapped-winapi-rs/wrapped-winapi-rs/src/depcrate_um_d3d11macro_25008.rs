// Generated macro for macro_25008 (macro)
macro_rules! Depcrate_um_d3d11macro_25008 {
() => {
// Module: crate::um::d3d11
// Provides: {"macro_25008"}
// Dependencies: {}
RIDL ! { # [uuid (0xa6cd7faa , 0xb0b7 , 0x4a2f , 0x94 , 0x36 , 0x86 , 0x62 , 0xa6 , 0x57 , 0x97 , 0xcb)] interface ID3D11ClassInstance (ID3D11ClassInstanceVtbl) : ID3D11DeviceChild (ID3D11DeviceChildVtbl) { fn GetClassLinkage (ppLinkage : * mut * mut ID3D11ClassLinkage ,) -> () , fn GetDesc (pDesc : * mut D3D11_CLASS_INSTANCE_DESC ,) -> () , fn GetInstanceName (pInstanceName : LPSTR , pBufferLength : * mut SIZE_T ,) -> () , fn GetTypeName (pTypeName : LPSTR , pBufferLength : * mut SIZE_T ,) -> () , } }
};
}
