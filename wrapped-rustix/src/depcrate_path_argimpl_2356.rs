// Generated macro for impl_2356 (impl)
macro_rules! Depcrate_path_argimpl_2356 {
() => {
// Module: crate::path::arg
// Provides: {"impl_2356"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Arg for & String { # [inline] fn as_str (& self) -> io :: Result < & str > { Ok (self) } # [cfg (feature = "alloc")] # [inline] fn to_string_lossy (& self) -> Cow < '_ , str > { Cow :: Borrowed (self) } # [cfg (feature = "alloc")] # [inline] fn as_cow_c_str (& self) -> io :: Result < Cow < '_ , CStr > > { Ok (Cow :: Owned (CString :: new (String :: as_str (self)) . map_err (| _cstr_err | io :: Errno :: INVAL) ? ,)) } # [cfg (feature = "alloc")] # [inline] fn into_c_str < 'b > (self) -> io :: Result < Cow < 'b , CStr > > where Self : 'b , { self . as_str () . into_c_str () } # [inline] fn into_with_c_str < T , F > (self , f : F) -> io :: Result < T > where Self : Sized , F : FnOnce (& CStr) -> io :: Result < T > , { with_c_str (self . as_bytes () , f) } }
};
}
