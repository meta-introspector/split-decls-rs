// Generated macro for path2bytes (function)
macro_rules! Depcrate_headerpath2bytes {
() => {
// Module: crate::header
// Provides: {"path2bytes"}
// Dependencies: {}
# [cfg (all (unix , not (target_arch = "wasm32")))] # [doc = " On unix this will never fail"] pub fn path2bytes (p : & Path) -> io :: Result < Cow < '_ , [u8] > > { Ok (Cow :: Borrowed (p . as_os_str () . as_bytes ())) }
};
}
