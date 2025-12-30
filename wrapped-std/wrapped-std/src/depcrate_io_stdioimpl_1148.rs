// Generated macro for impl_1148 (impl)
macro_rules! Depcrate_io_stdioimpl_1148 {
() => {
// Module: crate::io::stdio
// Provides: {"impl_1148"}
// Dependencies: {}
# [cfg (any (target_os = "linux" , target_os = "android"))] impl StdinLock < '_ > { pub (crate) fn as_mut_buf (& mut self) -> & mut BufReader < impl Read > { & mut self . inner } }
};
}
