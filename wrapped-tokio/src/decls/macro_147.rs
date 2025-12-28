macro_rules! deps {
    () => {
        ToSocketAddrs!();
    };
}

macro_rules! macro_147 {
    () => {
        deps!();
        cfg_net ! { pub (crate) fn to_socket_addrs < T > (arg : T) -> T :: Future where T : ToSocketAddrs , { arg . to_socket_addrs (sealed :: Internal) } }
    };
}

macro_147!();