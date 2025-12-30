// Generated macro for macro_26303 (macro)
macro_rules! Depcrate_um_d3d12macro_26303 {
() => {
// Module: crate::um::d3d12
// Provides: {"macro_26303"}
// Dependencies: {}
RIDL ! { # [uuid (0x7f91ce67 , 0x090c , 0x4bb7 , 0xb7 , 0x8e , 0xed , 0x8f , 0xf2 , 0xe3 , 0x1d , 0xa0)] interface ID3D12VersionedRootSignatureDeserializer (ID3D12VersionedRootSignatureDeserializerVtbl) : IUnknown (IUnknownVtbl) { fn GetRootSignatureDescAtVersion (convertToVersion : D3D_ROOT_SIGNATURE_VERSION , ppDesc : * mut * mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC ,) -> HRESULT , fn GetUnconvertedRootSignatureDesc () -> * const D3D12_VERSIONED_ROOT_SIGNATURE_DESC , } }
};
}
