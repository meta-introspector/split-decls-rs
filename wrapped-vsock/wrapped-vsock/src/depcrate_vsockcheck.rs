// Generated macro for check (function)
macro_rules! Depcrate_vsockcheck {
() => {
// Module: crate::vsock
// Provides: {"check"}
// Dependencies: {}
# [cfg (target_os = "hermit")] fn check < T : std :: ops :: Neg < Output = T > + std :: cmp :: PartialOrd < T > + IsNegative > (res : T ,) -> io :: Result < T > { if res . is_negative () { let e = match res . negate () { hermit_abi :: errno :: EACCES => std :: io :: ErrorKind :: PermissionDenied , hermit_abi :: errno :: EADDRINUSE => std :: io :: ErrorKind :: AddrInUse , hermit_abi :: errno :: EADDRNOTAVAIL => std :: io :: ErrorKind :: AddrNotAvailable , hermit_abi :: errno :: EAGAIN => std :: io :: ErrorKind :: WouldBlock , hermit_abi :: errno :: ECONNABORTED => std :: io :: ErrorKind :: ConnectionAborted , hermit_abi :: errno :: ECONNREFUSED => std :: io :: ErrorKind :: ConnectionRefused , hermit_abi :: errno :: ECONNRESET => std :: io :: ErrorKind :: ConnectionReset , hermit_abi :: errno :: EEXIST => std :: io :: ErrorKind :: AlreadyExists , hermit_abi :: errno :: EINTR => std :: io :: ErrorKind :: Interrupted , hermit_abi :: errno :: EINVAL => std :: io :: ErrorKind :: InvalidInput , hermit_abi :: errno :: ENOENT => std :: io :: ErrorKind :: NotFound , hermit_abi :: errno :: ENOTCONN => std :: io :: ErrorKind :: NotConnected , hermit_abi :: errno :: EPERM => std :: io :: ErrorKind :: PermissionDenied , hermit_abi :: errno :: EPIPE => std :: io :: ErrorKind :: BrokenPipe , hermit_abi :: errno :: ETIMEDOUT => std :: io :: ErrorKind :: TimedOut , _ => { println ! ("Unknown error number {}" , res . negate ()) ; std :: io :: ErrorKind :: InvalidInput } } ; Err (std :: io :: Error :: from (e)) } else { Ok (res) } }
};
}
