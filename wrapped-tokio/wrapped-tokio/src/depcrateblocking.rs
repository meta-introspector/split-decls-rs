// Generated macro for blocking (module)
macro_rules! Depcrateblocking {
() => {
// Module: crate
// Provides: {"blocking"}
// Dependencies: {}
# [cfg (any (feature = "fs" , feature = "io-std" , feature = "net" , all (windows , feature = "process") ,))] mod blocking ;
};
}
