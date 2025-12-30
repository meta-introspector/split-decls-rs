// Generated macro for impl_1198 (impl)
macro_rules! Depcrate_path_argimpl_1198 {
() => {
// Module: crate::path::arg
// Provides: {"impl_1198"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Arg for String { # [inline] fn as_str (& self) -> io :: Result < & str > { Ok (self) } # [cfg (feature = "alloc")] # [inline] fn to_string_lossy (& self) -> Cow < '_ , str > { Cow :: Borrowed (self) } # [cfg (feature = "alloc")] # [inline] fn as_cow_c_str (& self) -> io :: Result < Cow < '_ , CStr > > { Ok (Cow :: Owned (CString :: new (self . as_str ()) . map_err (| _cstr_err | io :: Errno :: INVAL) ? ,)) } # [cfg (feature = "alloc")] # [inline] fn into_c_str < 'b > (self) -> io :: Result < Cow < 'b , CStr > > where Self : 'b , { Ok (Cow :: Owned (CString :: new (self) . map_err (| _cstr_err | io :: Errno :: INVAL) ? ,)) } # [inline] fn into_with_c_str < T , F > (self , f : F) -> io :: Result < T > where Self : Sized , F : FnOnce (& CStr) -> io :: Result < T > , { f (& CString :: new (self) . map_err (| _cstr_err | io :: Errno :: INVAL) ?) } }
};
}
