// Generated macro for macro_33871 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33871 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33871"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000002 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IMalloc (IMallocVtbl) : IUnknown (IUnknownVtbl) { fn Alloc (cb : SIZE_T ,) -> * mut c_void , fn Realloc (pv : * mut c_void , cb : SIZE_T ,) -> * mut c_void , fn Free (pv : * mut c_void ,) -> () , fn GetSize (pv : * mut c_void ,) -> SIZE_T , fn DidAlloc (pv : * mut c_void ,) -> c_int , fn HeapMinimize () -> () , } }
};
}
