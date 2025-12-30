// Generated macro for macro_23945 (macro)
macro_rules! Depcrate_um_d2d1_1macro_23945 {
() => {
// Module: crate::um::d2d1_1
// Provides: {"macro_23945"}
// Dependencies: {}
RIDL ! { # [uuid (0x483473d7 , 0xcd46 , 0x4f9d , 0x9d , 0x3a , 0x31 , 0x12 , 0xaa , 0x80 , 0x15 , 0x9d)] interface ID2D1Properties (ID2D1PropertiesVtbl) : IUnknown (IUnknownVtbl) { fn GetPropertyCount () -> UINT32 , fn GetPropertyName (index : UINT32 , name : PWSTR , nameCount : UINT32 ,) -> HRESULT , fn GetPropertyNameLength (index : UINT32 ,) -> UINT32 , fn GetType (index : UINT32 ,) -> D2D1_PROPERTY_TYPE , fn GetPropertyIndex (name : PCWSTR ,) -> UINT32 , fn SetValueByName (name : PCWSTR , prop_type : D2D1_PROPERTY_TYPE , data : * const BYTE , dataSize : UINT32 ,) -> HRESULT , fn SetValue (index : UINT32 , prop_type : D2D1_PROPERTY_TYPE , data : * const BYTE , dataSize : UINT32 ,) -> HRESULT , fn GetValueByName (name : PCWSTR , prop_type : D2D1_PROPERTY_TYPE , data : * mut BYTE , dataSize : UINT32 ,) -> HRESULT , fn GetValue (index : UINT32 , prop_type : D2D1_PROPERTY_TYPE , data : * mut BYTE , dataSize : UINT32 ,) -> HRESULT , fn GetValueSize (index : UINT32 ,) -> UINT32 , fn GetSubProperties (index : UINT32 , subProperties : * mut * mut ID2D1Properties ,) -> HRESULT , } }
};
}
