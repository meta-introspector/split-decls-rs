// Generated macro for macro_25290 (macro)
macro_rules! Depcrate_um_d3d11_2macro_25290 {
() => {
// Module: crate::um::d3d11_2
// Provides: {"macro_25290"}
// Dependencies: {}
RIDL ! { # [uuid (0x9d06dffa , 0xd1e5 , 0x4d07 , 0x83 , 0xa8 , 0x1b , 0xb1 , 0x23 , 0xf2 , 0xf8 , 0x41)] interface ID3D11Device2 (ID3D11Device2Vtbl) : ID3D11Device1 (ID3D11Device1Vtbl) { fn GetImmediateContext2 (ppImmediateContext : * mut * mut ID3D11DeviceContext2 ,) -> () , fn CreateDeferredContext2 (ContextFlags : UINT , ppDeferredContext : * mut * mut ID3D11DeviceContext2 ,) -> HRESULT , fn GetResourceTiling (pTiledResource : * mut ID3D11Resource , pNumTilesForEntireResource : * mut UINT , pPackedMipDesc : * mut D3D11_PACKED_MIP_DESC , pStandardTileShapeForNonPackedMips : * mut D3D11_TILE_SHAPE , pNumSubresourceTilings : * mut UINT , FirstSubresourceTilingToGet : UINT , pSubresourceTilingsForNonPackedMips : * mut D3D11_SUBRESOURCE_TILING ,) -> () , fn CheckMultisampleQualityLevels1 (Format : DXGI_FORMAT , SampleCount : UINT , Flags : UINT , pNumQualityLevels : * mut UINT ,) -> HRESULT , } }
};
}
