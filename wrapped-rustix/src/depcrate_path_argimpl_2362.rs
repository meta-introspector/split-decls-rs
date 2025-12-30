// Generated macro for impl_2362 (impl)
macro_rules! Depcrate_path_argimpl_2362 {
() => {
// Module: crate::path::arg
// Provides: {"impl_2362"}
// Dependencies: {}
# [cfg (feature = "std")] impl Arg for & PathBuf { # [inline] fn as_str (& self) -> io :: Result < & str > { self . as_os_str () . to_str () . ok_or (io :: Errno :: INVAL) } # [inline] fn to_string_lossy (& self) -> Cow < '_ , str > { self . as_path () . to_string_lossy () } # [inline] fn as_cow_c_str (& self) -> io :: Result < Cow < '_ , CStr > > { self . as_os_str () . into_c_str () } # [inline] fn into_c_str < 'b > (self) -> io :: Result < Cow < 'b , CStr > > where Self : 'b , { self . as_path () . into_c_str () } # [inline] fn into_with_c_str < T , F > (self , f : F) -> io :: Result < T > where Self : Sized , F : FnOnce (& CStr) -> io :: Result < T > , { self . as_os_str () . into_with_c_str (f) } }
};
}
