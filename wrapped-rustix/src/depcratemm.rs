// Generated macro for mm (module)
macro_rules! Depcratemm {
() => {
// Module: crate
// Provides: {"mm"}
// Dependencies: {}
# [cfg (not (any (windows , target_os = "espidf" , target_os = "horizon" , target_os = "vita" , target_os = "wasi")))] # [cfg (feature = "mm")] # [cfg_attr (docsrs , doc (cfg (feature = "mm")))] pub mod mm ;
};
}
