// Generated macro for impl_1217 (impl)
macro_rules! Depcrate_path_argimpl_1217 {
() => {
// Module: crate::path::arg
// Provides: {"impl_1217"}
// Dependencies: {}
impl Arg for DecInt { # [inline] fn as_str (& self) -> io :: Result < & str > { Ok (self . as_str ()) } # [cfg (feature = "alloc")] # [inline] fn to_string_lossy (& self) -> Cow < '_ , str > { Cow :: Borrowed (self . as_str ()) } # [cfg (feature = "alloc")] # [inline] fn as_cow_c_str (& self) -> io :: Result < Cow < '_ , CStr > > { Ok (Cow :: Borrowed (self . as_c_str ())) } # [cfg (feature = "alloc")] # [inline] fn into_c_str < 'b > (self) -> io :: Result < Cow < 'b , CStr > > where Self : 'b , { Ok (Cow :: Owned (self . as_c_str () . to_owned ())) } # [inline] fn into_with_c_str < T , F > (self , f : F) -> io :: Result < T > where Self : Sized , F : FnOnce (& CStr) -> io :: Result < T > , { f (self . as_c_str ()) } }
};
}
