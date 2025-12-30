// Generated macro for macro_27220 (macro)
macro_rules! Depcrate_um_dcompmacro_27220 {
() => {
// Module: crate::um::dcomp
// Provides: {"macro_27220"}
// Dependencies: {}
RIDL ! { # [uuid (0x9b7e82e2 , 0x69c5 , 0x4eb4 , 0xa5 , 0xf5 , 0xa7 , 0x03 , 0x3f , 0x51 , 0x32 , 0xcd)] interface IDCompositionTableTransferEffect (IDCompositionTableTransferEffectVtbl) : IDCompositionFilterEffect (IDCompositionFilterEffectVtbl) { fn SetRedTable (tableValues : * const c_float , count : UINT ,) -> HRESULT , fn SetGreenTable (tableValues : * const c_float , count : UINT ,) -> HRESULT , fn SetBlueTable (tableValues : * const c_float , count : UINT ,) -> HRESULT , fn SetAlphaTable (tableValues : * const c_float , count : UINT ,) -> HRESULT , fn SetRedDisable (redDisable : BOOL ,) -> HRESULT , fn SetGreenDisable (greenDisable : BOOL ,) -> HRESULT , fn SetBlueDisable (blueDisable : BOOL ,) -> HRESULT , fn SetAlphaDisable (alphaDisable : BOOL ,) -> HRESULT , fn SetClampOutput (clampOutput : BOOL ,) -> HRESULT , fn SetRedTableValue_2 (index : UINT , animation : * const IDCompositionAnimation ,) -> HRESULT , fn SetRedTableValue_1 (index : UINT , value : c_float ,) -> HRESULT , fn SetGreenTableValue_2 (index : UINT , animation : * const IDCompositionAnimation ,) -> HRESULT , fn SetGreenTableValue_1 (index : UINT , value : c_float ,) -> HRESULT , fn SetBlueTableValue_2 (index : UINT , animation : * const IDCompositionAnimation ,) -> HRESULT , fn SetBlueTableValue_1 (index : UINT , value : c_float ,) -> HRESULT , fn SetAlphaTableValue_2 (index : UINT , animation : * const IDCompositionAnimation ,) -> HRESULT , fn SetAlphaTableValue_1 (index : UINT , value : c_float ,) -> HRESULT , } }
};
}
