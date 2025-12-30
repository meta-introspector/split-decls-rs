// Generated macro for sys (module)
macro_rules! Depcratesys {
() => {
// Module: crate
// Provides: {"sys"}
// Dependencies: {}
# [cfg_attr (unix , path = "sys/unix.rs")] # [cfg_attr (windows , path = "sys/windows.rs")] mod sys ;
};
}
