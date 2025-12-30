// Generated macro for macro_3395 (macro)
macro_rules! Depcrate_shared_dxgi1_2macro_3395 {
() => {
// Module: crate::shared::dxgi1_2
// Provides: {"macro_3395"}
// Dependencies: {}
RIDL ! { # [uuid (0x191cfac3 , 0xa341 , 0x470d , 0xb2 , 0x6e , 0xa8 , 0x64 , 0xf4 , 0x28 , 0x31 , 0x9c)] interface IDXGIOutputDuplication (IDXGIOutputDuplicationVtbl) : IDXGIObject (IDXGIObjectVtbl) { fn GetDesc (pDesc : * mut DXGI_OUTDUPL_DESC ,) -> () , fn AcquireNextFrame (TimeoutInMilliseconds : UINT , pFrameInfo : * mut DXGI_OUTDUPL_FRAME_INFO , ppDesktopResource : * mut * mut IDXGIResource ,) -> HRESULT , fn GetFrameDirtyRects (DirtyRectsBufferSize : UINT , pDirtyRectsBuffer : * mut RECT , pDirtyRectsBufferSizeRequired : * mut UINT ,) -> HRESULT , fn GetFrameMoveRects (MoveRectsBufferSize : UINT , pMoveRectBuffer : * mut DXGI_OUTDUPL_MOVE_RECT , pMoveRectsBufferSizeRequired : * mut UINT ,) -> HRESULT , fn GetFramePointerShape (PointerShapeBufferSize : UINT , pPointerShapeBuffer : * mut c_void , pPointerShapeBufferSizeRequired : * mut UINT , pPointerShapeInfo : * mut DXGI_OUTDUPL_POINTER_SHAPE_INFO ,) -> HRESULT , fn MapDesktopSurface (pLockedRect : * mut DXGI_MAPPED_RECT ,) -> HRESULT , fn UnMapDesktopSurface () -> HRESULT , fn ReleaseFrame () -> HRESULT , } }
};
}
