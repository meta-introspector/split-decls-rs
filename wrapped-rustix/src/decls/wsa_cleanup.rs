macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! wsa_cleanup {
    () => {
        deps!();
        # [doc = " `WSACleanup()`—Clean up process-wide Windows support for sockets."] # [doc = ""] # [doc = " In a program where `init` is called, if sockets are no longer necessary,"] # [doc = " this function releases associated resources."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Winsock]"] # [doc = ""] # [doc = " [Winsock]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsacleanup"] pub fn wsa_cleanup () -> io :: Result < () > { unsafe { if WSACleanup () == 0 { Ok (()) } else { Err (io :: Errno :: from_raw_os_error (WSAGetLastError ())) } } }
    };
}

wsa_cleanup!()