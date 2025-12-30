// Generated macro for SysvarId (trait)
macro_rules! DepcrateSysvarId {
() => {
// Module: crate
// Provides: {"SysvarId"}
// Dependencies: {}
# [doc = " A type that holds sysvar data and has an associated sysvar `Address`."] pub trait SysvarId { # [doc = " The `Address` of the sysvar."] fn id () -> Address ; # [doc = " Returns `true` if the given address is the ID."] fn check_id (address : & Address) -> bool ; }
};
}
