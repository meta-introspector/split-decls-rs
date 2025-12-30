// Generated macro for cmsg_aligned_space (macro)
macro_rules! Depcrate_net_send_recv_msgcmsg_aligned_space {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"cmsg_aligned_space"}
// Dependencies: {}
# [doc = " Like `cmsg_space`, but doesn't add padding for `cmsghdr` alignment."] # [doc (hidden)] # [macro_export] macro_rules ! cmsg_aligned_space { (ScmRights ($ len : expr)) => { $ crate :: net :: __cmsg_aligned_space ($ len * :: core :: mem :: size_of ::<$ crate :: fd :: BorrowedFd <'static >> () ,) } ; (ScmCredentials ($ len : expr)) => { $ crate :: net :: __cmsg_aligned_space ($ len * :: core :: mem :: size_of ::<$ crate :: net :: UCred > () ,) } ; (TxTime ($ len : expr)) => { $ crate :: net :: __cmsg_aligned_space ($ len * :: core :: mem :: size_of ::<:: core :: primitive :: u64 > () ,) } ; ($ firstid : ident ($ firstex : expr) , $ ($ restid : ident ($ restex : expr)) ,*) => { { let sum = $ crate :: cmsg_aligned_space ! ($ firstid ($ firstex)) ; $ (let sum = sum + $ crate :: cmsg_aligned_space ! ($ restid ($ restex)) ;) * sum } } ; }
};
}
