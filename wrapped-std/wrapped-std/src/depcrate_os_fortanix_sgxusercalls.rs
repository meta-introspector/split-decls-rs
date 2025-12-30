// Generated macro for usercalls (module)
macro_rules! Depcrate_os_fortanix_sgxusercalls {
() => {
// Module: crate::os::fortanix_sgx
// Provides: {"usercalls"}
// Dependencies: {}
# [doc = " Low-level interfaces to usercalls. See the [ABI documentation] for more"] # [doc = " information."] # [doc = ""] # [doc = " [ABI documentation]: https://docs.rs/fortanix-sgx-abi/"] pub mod usercalls { pub use crate :: sys :: abi :: usercalls :: * ; # [doc = " Primitives for allocating memory in userspace as well as copying data"] # [doc = " to and from user memory."] pub mod alloc { pub use crate :: sys :: abi :: usercalls :: alloc :: * ; } # [doc = " Lowest-level interfaces to usercalls and usercall ABI type definitions."] pub mod raw { pub use crate :: sys :: abi :: usercalls :: raw :: { ByteBuffer , Cancel , EV_RETURNQ_NOT_EMPTY , EV_UNPARK , EV_USERCALLQ_NOT_FULL , Error , FD_STDERR , FD_STDIN , FD_STDOUT , Fd , FifoDescriptor , RESULT_SUCCESS , Register , RegisterArgument , Result , Return , ReturnValue , Tcs , USERCALL_USER_DEFINED , Usercall , Usercalls as UsercallNrs , WAIT_INDEFINITE , WAIT_NO , accept_stream , alloc , async_queues , bind_stream , close , connect_stream , do_usercall , exit , flush , free , insecure_time , launch_thread , read , read_alloc , send , wait , write , } ; } }
};
}
