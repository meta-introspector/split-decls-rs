// Generated macro for UCred (struct)
macro_rules! Depcrate_os_unix_net_ucredUCred {
() => {
// Module: crate::os::unix::net::ucred
// Provides: {"UCred"}
// Dependencies: {}
# [doc = " Credentials for a UNIX process for credentials passing."] # [unstable (feature = "peer_credentials_unix_socket" , issue = "42839" , reason = "unstable")] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub struct UCred { # [doc = " The UID part of the peer credential. This is the effective UID of the process at the domain"] # [doc = " socket's endpoint."] pub uid : uid_t , # [doc = " The GID part of the peer credential. This is the effective GID of the process at the domain"] # [doc = " socket's endpoint."] pub gid : gid_t , # [doc = " The PID part of the peer credential. This field is optional because the PID part of the"] # [doc = " peer credentials is not supported on every platform. On platforms where the mechanism to"] # [doc = " discover the PID exists, this field will be populated to the PID of the process at the"] # [doc = " domain socket's endpoint. Otherwise, it will be set to None."] pub pid : Option < pid_t > , }
};
}
