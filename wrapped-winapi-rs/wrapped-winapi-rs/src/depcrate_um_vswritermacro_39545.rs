// Generated macro for macro_39545 (macro)
macro_rules! Depcrate_um_vswritermacro_39545 {
() => {
// Module: crate::um::vswriter
// Provides: {"macro_39545"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000000 , 0x0000 , 0x0000 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00)] interface IVssWriterComponents (IVssWriterComponentsVtbl) { fn GetComponentCount (pcComponents : * mut UINT ,) -> HRESULT , fn GetWriterInfo (pidInstance : * mut VSS_ID , pidWriter : * mut VSS_ID ,) -> HRESULT , fn GetComponent (iComponent : UINT , ppComponent : * mut * mut IVssComponent ,) -> HRESULT , } }
};
}
