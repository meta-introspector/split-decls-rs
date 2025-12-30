// Generated macro for impl_1518 (impl)
macro_rules! Depcrate_os_unix_net_ancillaryimpl_1518 {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"impl_1518"}
// Dependencies: {}
# [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] impl < 'a > Iterator for Messages < 'a > { type Item = Result < AncillaryData < 'a > , AncillaryError > ; fn next (& mut self) -> Option < Self :: Item > { unsafe { let mut msg : libc :: msghdr = zeroed () ; msg . msg_control = self . buffer . as_ptr () as * mut _ ; msg . msg_controllen = self . buffer . len () as _ ; let cmsg = if let Some (current) = self . current { libc :: CMSG_NXTHDR (& msg , current) } else { libc :: CMSG_FIRSTHDR (& msg) } ; let cmsg = cmsg . as_ref () ? ; if let Some (current) = self . current { if eq (current , cmsg) { return None ; } } self . current = Some (cmsg) ; let ancillary_result = AncillaryData :: try_from_cmsghdr (cmsg) ; Some (ancillary_result) } } }
};
}
