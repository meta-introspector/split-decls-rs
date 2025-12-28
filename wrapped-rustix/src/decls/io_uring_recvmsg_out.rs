macro_rules! deps {
    () => {
        SocketAddrLen!();
    };
}

macro_rules! io_uring_recvmsg_out {
    () => {
        deps!();
        # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Default , Copy , Clone)] pub struct io_uring_recvmsg_out { pub namelen : SocketAddrLen , pub controllen : u32 , pub payloadlen : u32 , pub flags : RecvmsgOutFlags , }
    };
}

io_uring_recvmsg_out!();