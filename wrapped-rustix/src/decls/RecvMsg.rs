macro_rules! deps {
    () => {
        SocketAddrAny!();
    };
}

macro_rules! RecvMsg {
    () => {
        deps!();
        # [doc = " The result of a successful [`recvmsg`] call."] # [derive (Debug , Clone)] pub struct RecvMsg { # [doc = " The number of bytes received."] # [doc = ""] # [doc = " When `RecvFlags::TRUNC` is in use, this may be greater than the length"] # [doc = " of the buffer, as it reflects the number of bytes received before"] # [doc = " truncation into the buffer."] pub bytes : usize , # [doc = " The flags received."] pub flags : ReturnFlags , # [doc = " The address of the socket we received from, if any."] pub address : Option < SocketAddrAny > , }
    };
}

RecvMsg!();