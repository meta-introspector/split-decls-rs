// Generated macro for macro_35957 (macro)
macro_rules! Depcrate_um_propsysmacro_35957 {
() => {
// Module: crate::um::propsys
// Provides: {"macro_35957"}
// Dependencies: {}
RIDL ! { # [uuid (0x886d8eeb , 0x8cf2 , 0x4446 , 0x8d , 0x02 , 0xcd , 0xba , 0x1d , 0xbd , 0xcf , 0x99)] interface IPropertyStore (IPropertyStoreVtbl) : IUnknown (IUnknownVtbl) { fn GetCount (cProps : * mut DWORD ,) -> HRESULT , fn GetAt (iProp : DWORD , pkey : * mut PROPERTYKEY ,) -> HRESULT , fn GetValue (key : REFPROPERTYKEY , pv : * mut PROPVARIANT ,) -> HRESULT , fn SetValue (key : REFPROPERTYKEY , propvar : REFPROPVARIANT ,) -> HRESULT , fn Commit () -> HRESULT , } }
};
}
