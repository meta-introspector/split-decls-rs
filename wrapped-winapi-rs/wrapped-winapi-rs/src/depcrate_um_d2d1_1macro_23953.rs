// Generated macro for macro_23953 (macro)
macro_rules! Depcrate_um_d2d1_1macro_23953 {
() => {
// Module: crate::um::d2d1_1
// Provides: {"macro_23953"}
// Dependencies: {}
RIDL ! { # [uuid (0xbb12d362 , 0xdaee , 0x4b9a , 0xaa , 0x1d , 0x14 , 0xba , 0x40 , 0x1c , 0xfa , 0x1f)] interface ID2D1Factory1 (ID2D1Factory1Vtbl) : ID2D1Factory (ID2D1FactoryVtbl) { fn CreateDevice (dxgiDevice : * const IDXGIDevice , d2dDevice : * mut * mut ID2D1Device ,) -> HRESULT , fn CreateStrokeStyle (strokeStyleProperties : * const D2D1_STROKE_STYLE_PROPERTIES1 , dashes : * const FLOAT , dashesCount : UINT32 , strokeStyle : * mut * mut ID2D1StrokeStyle1 ,) -> HRESULT , fn CreatePathGeometry (pathGeometry : * mut * mut ID2D1PathGeometry1 ,) -> HRESULT , fn CreateDrawingStateBlock (drawingStateDescription : * const D2D1_DRAWING_STATE_DESCRIPTION1 , textRenderingParams : * const IDWriteRenderingParams , drawingStateBlock : * mut * mut ID2D1DrawingStateBlock1 ,) -> HRESULT , fn CreateGdiMetafile (metafileStream : * const IStream , metafile : * mut * mut ID2D1GdiMetafile ,) -> HRESULT , fn RegisterEffectFromStream (classId : REFCLSID , propertyXml : * const IStream , bindings : * const D2D1_PROPERTY_BINDING , bindingsCount : UINT32 , effectFactory : PD2D1_EFFECT_FACTORY ,) -> HRESULT , fn RegisterEffectFromString (classId : REFCLSID , propertyXml : PCWSTR , bindings : * const D2D1_PROPERTY_BINDING , bindingsCount : UINT32 , effectFactory : PD2D1_EFFECT_FACTORY ,) -> HRESULT , fn UnregisterEffect (classId : REFCLSID ,) -> HRESULT , fn GetRegisteredEffects (effects : * mut CLSID , effectsCount : UINT32 , effectsReturned : * mut UINT32 , effectsRegistered : * mut UINT32 ,) -> HRESULT , fn GetEffectProperties (effectId : REFCLSID , properties : * mut * mut ID2D1Properties ,) -> HRESULT , } }
};
}
