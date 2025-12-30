// Generated macro for Messages (struct)
macro_rules! Depcrate_os_unix_net_ancillaryMessages {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"Messages"}
// Dependencies: {}
# [doc = " This struct is used to iterate through the control messages."] # [must_use = "iterators are lazy and do nothing unless consumed"] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub struct Messages < 'a > { buffer : & 'a [u8] , current : Option < & 'a libc :: cmsghdr > , }
};
}
