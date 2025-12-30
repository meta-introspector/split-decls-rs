// Generated macro for dlsym_fn (macro)
macro_rules! Depcrate_sys_platform_version_darwin_core_foundationdlsym_fn {
() => {
// Module: crate::sys::platform_version::darwin::core_foundation
// Provides: {"dlsym_fn"}
// Dependencies: {}
macro_rules ! dlsym_fn { (unsafe fn $ name : ident ($ ($ param : ident : $ param_ty : ty) ,* $ (,) ?) $ (-> $ ret : ty) ?;) => { pub (super) unsafe fn $ name (& self , $ ($ param : $ param_ty) ,*) $ (-> $ ret) ? { let ptr = unsafe { libc :: dlsym (self . 0 , concat ! (stringify ! ($ name) , '\0') . as_bytes () . as_ptr () . cast () ,) } ; if ptr . is_null () { let err = unsafe { CStr :: from_ptr (libc :: dlerror ()) } ; panic ! ("could not find function {}: {err:?}" , stringify ! ($ name)) ; } let fnptr = unsafe { crate :: mem :: transmute ::< * mut c_void , unsafe extern "C" fn ($ ($ param_ty) ,*) $ (-> $ ret) ?, > (ptr) } ; unsafe { fnptr ($ ($ param) ,*) } } } ; }
};
}
