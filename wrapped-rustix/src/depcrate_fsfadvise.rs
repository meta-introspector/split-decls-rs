// Generated macro for fadvise (module)
macro_rules! Depcrate_fsfadvise {
() => {
// Module: crate::fs
// Provides: {"fadvise"}
// Dependencies: {}
# [cfg (not (any (apple , netbsdlike , target_os = "dragonfly" , target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "redox" , target_os = "solaris" , target_os = "vita" ,)))] mod fadvise ;
};
}
