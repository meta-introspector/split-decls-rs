macro_rules! deps {
    () => {
        Domain!();
        Type!();
    };
}

macro_rules! Socket {
    () => {
        deps!();
        # [doc = " Owned wrapper around a system socket."] # [doc = ""] # [doc = " This type simply wraps an instance of a file descriptor (`c_int`) on Unix"] # [doc = " and an instance of `SOCKET` on Windows. This is the main type exported by"] # [doc = " this crate and is intended to mirror the raw semantics of sockets on"] # [doc = " platforms as closely as possible. Almost all methods correspond to"] # [doc = " precisely one libc or OS API call which is essentially just a \"Rustic"] # [doc = " translation\" of what's below."] # [doc = ""] # [doc = " ## Converting to and from other types"] # [doc = ""] # [doc = " This type can be freely converted into the network primitives provided by"] # [doc = " the standard library, such as [`TcpStream`] or [`UdpSocket`], using the"] # [doc = " [`From`] trait, see the example below."] # [doc = ""] # [doc = " [`TcpStream`]: std::net::TcpStream"] # [doc = " [`UdpSocket`]: std::net::UdpSocket"] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Some methods that set options on `Socket` require two system calls to set"] # [doc = " their options without overwriting previously set options. We do this by"] # [doc = " first getting the current settings, applying the desired changes, and then"] # [doc = " updating the settings. This means that the operation is **not** atomic. This"] # [doc = " can lead to a data race when two threads are changing options in parallel."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```no_run"] # [doc = " # fn main() -> std::io::Result<()> {"] # [doc = " use std::net::{SocketAddr, TcpListener};"] # [doc = " use socket2::{Socket, Domain, Type};"] # [doc = ""] # [doc = " // create a TCP listener"] # [doc = " let socket = Socket::new(Domain::IPV6, Type::STREAM, None)?;"] # [doc = ""] # [doc = " let address: SocketAddr = \"[::1]:12345\".parse().unwrap();"] # [doc = " let address = address.into();"] # [doc = " socket.bind(&address)?;"] # [doc = " socket.listen(128)?;"] # [doc = ""] # [doc = " let listener: TcpListener = socket.into();"] # [doc = " // ..."] # [doc = " # drop(listener);"] # [doc = " # Ok(()) }"] # [doc = " ```"] pub struct Socket { inner : sys :: Socket , }
    };
}

Socket!()