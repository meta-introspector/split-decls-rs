macro_rules! deps {
    () => {
        MMsgHdr!();
        Result!();
    };
}

macro_rules! sendmmsg {
    () => {
        deps!();
        # [doc = " `sendmmsg(msghdr)`—Sends multiple messages on a socket."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/sendmmsg.2.html"] # [inline] # [cfg (target_os = "linux")] pub fn sendmmsg < Fd : AsFd > (socket : Fd , msgs : & mut [MMsgHdr < '_ >] , flags : SendFlags ,) -> io :: Result < usize > { backend :: net :: syscalls :: sendmmsg (socket . as_fd () , msgs , flags) }
    };
}

sendmmsg!()