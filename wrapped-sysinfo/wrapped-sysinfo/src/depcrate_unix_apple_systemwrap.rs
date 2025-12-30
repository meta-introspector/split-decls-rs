// Generated macro for Wrap (struct)
macro_rules! Depcrate_unix_apple_systemWrap {
() => {
// Module: crate::unix::apple::system
// Provides: {"Wrap"}
// Dependencies: {}
# [cfg (all (target_os = "macos" , not (feature = "apple-sandbox")))] pub (crate) struct Wrap < 'a > (pub UnsafeCell < & 'a mut HashMap < Pid , Process > >) ;
};
}
