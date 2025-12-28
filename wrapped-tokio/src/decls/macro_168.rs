macro_rules! macro_168 {
    () => {
        cfg_net ! { mod lookup_host ; pub use lookup_host :: lookup_host ; pub mod tcp ; pub use tcp :: listener :: TcpListener ; pub use tcp :: stream :: TcpStream ; cfg_not_wasi ! { pub use tcp :: socket :: TcpSocket ; mod udp ; # [doc (inline)] pub use udp :: UdpSocket ; } }
    };
}

macro_168!()