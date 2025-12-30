// Generated macro for shm (module)
macro_rules! Depcrateshm {
() => {
// Module: crate
// Provides: {"shm"}
// Dependencies: {}
# [cfg (not (any (windows , target_os = "android" , target_os = "espidf" , target_os = "horizon" , target_os = "vita" , target_os = "wasi")))] # [cfg (feature = "shm")] # [cfg_attr (docsrs , doc (cfg (feature = "shm")))] pub mod shm ;
};
}
