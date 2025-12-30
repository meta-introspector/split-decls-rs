// Generated macro for clockid (module)
macro_rules! Depcrateclockid {
() => {
// Module: crate
// Provides: {"clockid"}
// Dependencies: {}
# [cfg (not (any (windows , target_os = "espidf")))] # [cfg (any (feature = "thread" , feature = "time"))] mod clockid ;
};
}
