macro_rules! deps {
    () => {
        SocketAddrLen!();
    };
}

macro_rules! MsgHdr {
    () => {
        deps!();
        # [doc = " `msghdr`"] # [allow (missing_docs)] # [repr (C)] pub struct MsgHdr { pub msg_name : * mut c_void , pub msg_namelen : SocketAddrLen , pub msg_iov : * mut iovec , pub msg_iovlen : usize , pub msg_control : * mut c_void , pub msg_controllen : usize , pub msg_flags : RecvFlags , }
    };
}

MsgHdr!()