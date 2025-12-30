// Generated macro for tests (module)
macro_rules! Depcrate_net_send_recv_msgtests {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [no_implicit_prelude] mod hygiene { # [allow (unused_macros)] # [test] fn macro_hygiene () { # [allow (dead_code , non_camel_case_types)] struct u64 ([u8]) ; macro_rules ! cmsg_space { ($ ($ tt : tt) *) => { { let v : usize = :: core :: panic ! ("Wrong cmsg_space! macro called") ; v } } ; } macro_rules ! cmsg_aligned_space { ($ ($ tt : tt) *) => { { let v : usize = :: core :: panic ! ("Wrong cmsg_aligned_space! macro called") ; v } } ; } crate :: cmsg_space ! (ScmRights (1)) ; crate :: cmsg_space ! (TxTime (1)) ; # [cfg (linux_kernel)] { crate :: cmsg_space ! (ScmCredentials (1)) ; crate :: cmsg_space ! (ScmRights (1) , ScmCredentials (1) , TxTime (1)) ; crate :: cmsg_aligned_space ! (ScmRights (1) , ScmCredentials (1) , TxTime (1)) ; } } } }
};
}
