// Generated macro for macro_40650 (macro)
macro_rules! Depcrate_um_wincodecsdkmacro_40650 {
() => {
// Module: crate::um::wincodecsdk
// Provides: {"macro_40650"}
// Dependencies: {}
RIDL ! { # [uuid (0xf7836e16 , 0x3be0 , 0x470b , 0x86 , 0xbb , 0x16 , 0x0d , 0x0a , 0xec , 0xd7 , 0xde)] interface IWICMetadataWriter (IWICMetadataWriterVtbl) : IWICMetadataReader (IWICMetadataReaderVtbl) { fn SetValue (pvarSchema : * const PROPVARIANT , pvarId : * const PROPVARIANT , pvarValue : * const PROPVARIANT ,) -> HRESULT , fn SetValueByIndex (nIndex : UINT , pvarSchema : * const PROPVARIANT , pvarId : * const PROPVARIANT , pvarValue : * const PROPVARIANT ,) -> HRESULT , fn RemoveValue (pvarSchema : * const PROPVARIANT , pvarId : * const PROPVARIANT ,) -> HRESULT , fn RemoveValueByIndex (nIndex : UINT ,) -> HRESULT , } }
};
}
