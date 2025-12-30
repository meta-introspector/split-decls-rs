// Generated macro for AncillaryError (enum)
macro_rules! Depcrate_os_unix_net_ancillaryAncillaryError {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"AncillaryError"}
// Dependencies: {}
# [doc = " The error type which is returned from parsing the type a control message."] # [non_exhaustive] # [derive (Debug)] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub enum AncillaryError { Unknown { cmsg_level : i32 , cmsg_type : i32 } , }
};
}
