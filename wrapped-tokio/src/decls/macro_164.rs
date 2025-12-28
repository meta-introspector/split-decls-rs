macro_rules! deps {
    () => {
        ToSocketAddrs!();
    };
}

macro_rules! macro_164 {
    () => {
        deps!();
        cfg_net ! { impl ToSocketAddrs for str { } impl sealed :: ToSocketAddrsPriv for str { type Iter = sealed :: OneOrMore ; type Future = sealed :: MaybeReady ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { use crate :: blocking :: spawn_blocking ; use sealed :: MaybeReady ; let res : Result < SocketAddr , _ > = self . parse () ; if let Ok (addr) = res { return MaybeReady (sealed :: State :: Ready (Some (addr))) ; } let s = self . to_owned () ; MaybeReady (sealed :: State :: Blocking (spawn_blocking (move || { std :: net :: ToSocketAddrs :: to_socket_addrs (& s) }))) } } impl ToSocketAddrs for (& str , u16) { } impl sealed :: ToSocketAddrsPriv for (& str , u16) { type Iter = sealed :: OneOrMore ; type Future = sealed :: MaybeReady ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { use crate :: blocking :: spawn_blocking ; use sealed :: MaybeReady ; let (host , port) = * self ; if let Ok (addr) = host . parse ::< Ipv4Addr > () { let addr = SocketAddrV4 :: new (addr , port) ; let addr = SocketAddr :: V4 (addr) ; return MaybeReady (sealed :: State :: Ready (Some (addr))) ; } if let Ok (addr) = host . parse ::< Ipv6Addr > () { let addr = SocketAddrV6 :: new (addr , port , 0 , 0) ; let addr = SocketAddr :: V6 (addr) ; return MaybeReady (sealed :: State :: Ready (Some (addr))) ; } let host = host . to_owned () ; MaybeReady (sealed :: State :: Blocking (spawn_blocking (move || { std :: net :: ToSocketAddrs :: to_socket_addrs (& (& host [..] , port)) }))) } } impl ToSocketAddrs for (String , u16) { } impl sealed :: ToSocketAddrsPriv for (String , u16) { type Iter = sealed :: OneOrMore ; type Future = sealed :: MaybeReady ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { (self . 0 . as_str () , self . 1) . to_socket_addrs (sealed :: Internal) } } impl ToSocketAddrs for String { } impl sealed :: ToSocketAddrsPriv for String { type Iter = < str as sealed :: ToSocketAddrsPriv >:: Iter ; type Future = < str as sealed :: ToSocketAddrsPriv >:: Future ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { self [..] . to_socket_addrs (sealed :: Internal) } } }
    };
}

macro_164!()