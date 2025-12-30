// Generated macro for impl_114 (impl)
macro_rules! Depcrate_errorimpl_114 {
() => {
// Module: crate::error
// Provides: {"impl_114"}
// Dependencies: {}
impl Error { pub fn new (errno : i32) -> Error { Error { errno } } pub fn mux (result : Result < usize >) -> usize { match result { Ok (value) => value , Err (error) => - error . errno as usize , } } pub fn demux (value : usize) -> Result < usize > { let errno = - (value as i32) ; if errno >= 1 && errno < STR_ERROR . len () as i32 { Err (Error :: new (errno)) } else { Ok (value) } } pub fn text (& self) -> & 'static str { STR_ERROR . get (self . errno as usize) . map (| & x | x) . unwrap_or ("Unknown Error") } }
};
}
