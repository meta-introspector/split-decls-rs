// Generated macro for macro_23768 (macro)
macro_rules! Depcrate_um_corsymmacro_23768 {
() => {
// Module: crate::um::corsym
// Provides: {"macro_23768"}
// Dependencies: {}
RIDL ! { # [uuid (0xaa544d42 , 0x28cb , 0x11d3 , 0xbd , 0x22 , 0x00 , 0x00 , 0xf8 , 0x08 , 0x49 , 0xbd)] interface ISymUnmanagedBinder (ISymUnmanagedBinderVtbl) : IUnknown (IUnknownVtbl) { fn GetReaderForFile (importer : * mut IUnknown , fileName : * const WCHAR , searchPath : * const WCHAR , pRetVal : * mut * mut ISymUnmanagedReader ,) -> HRESULT , fn GetReaderFromStream (importer : * mut IUnknown , pstream : * mut IStream , pRetVal : * mut * mut ISymUnmanagedReader ,) -> HRESULT , } }
};
}
