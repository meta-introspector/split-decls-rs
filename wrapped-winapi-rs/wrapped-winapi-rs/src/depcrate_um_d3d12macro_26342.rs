// Generated macro for macro_26342 (macro)
macro_rules! Depcrate_um_d3d12macro_26342 {
() => {
// Module: crate::um::d3d12
// Provides: {"macro_26342"}
// Dependencies: {}
RIDL ! { # [uuid (0x0a753dcf , 0xc4d8 , 0x4b91 , 0xad , 0xf6 , 0xbe , 0x5a , 0x60 , 0xd9 , 0x5a , 0x76)] interface ID3D12Fence (ID3D12FenceVtbl) : ID3D12Pageable (ID3D12PageableVtbl) { fn GetCompletedValue () -> UINT64 , fn SetEventOnCompletion (Value : UINT64 , hEvent : HANDLE ,) -> HRESULT , fn Signal (Value : UINT64 ,) -> HRESULT , } }
};
}
