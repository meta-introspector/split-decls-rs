// Generated macro for impl_2369 (impl)
macro_rules! Depcrate_path_argimpl_2369 {
() => {
// Module: crate::path::arg
// Provides: {"impl_2369"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a > Arg for Cow < 'a , CStr > { # [inline] fn as_str (& self) -> io :: Result < & str > { self . to_str () . map_err (| _utf8_err | io :: Errno :: INVAL) } # [inline] fn to_string_lossy (& self) -> Cow < '_ , str > { let borrow : & CStr = core :: borrow :: Borrow :: borrow (self) ; borrow . to_string_lossy () } # [inline] fn as_cow_c_str (& self) -> io :: Result < Cow < '_ , CStr > > { Ok (Cow :: Borrowed (self)) } # [inline] fn into_c_str < 'b > (self) -> io :: Result < Cow < 'b , CStr > > where Self : 'b , { Ok (self) } # [inline] fn into_with_c_str < T , F > (self , f : F) -> io :: Result < T > where Self : Sized , F : FnOnce (& CStr) -> io :: Result < T > , { f (& self) } }
};
}
