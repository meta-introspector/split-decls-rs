// Generated macro for impl_2979 (impl)
macro_rules! Depcrate_processimpl_2979 {
() => {
// Module: crate::process
// Provides: {"impl_2979"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl fmt :: Debug for Command { # [doc = " Format the program and arguments of a Command for display. Any"] # [doc = " non-utf8 data is lossily converted using the utf8 replacement"] # [doc = " character."] # [doc = ""] # [doc = " The default format approximates a shell invocation of the program along with its"] # [doc = " arguments. It does not include most of the other command properties. The output is not guaranteed to work"] # [doc = " (e.g. due to lack of shell-escaping or differences in path resolution)."] # [doc = " On some platforms you can use [the alternate syntax] to show more fields."] # [doc = ""] # [doc = " Note that the debug implementation is platform-specific."] # [doc = ""] # [doc = " [the alternate syntax]: fmt#sign0"] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . inner . fmt (f) } }
};
}
