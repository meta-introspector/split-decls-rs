macro_rules! deps {
    () => {
        RawProtocol!();
    };
}

macro_rules! new_raw_protocol {
    () => {
        deps!();
        const fn new_raw_protocol (u : u32) -> RawProtocol { match RawProtocol :: new (u) { Some (p) => p , None => panic ! ("new_raw_protocol: protocol must be non-zero") , } }
    };
}

new_raw_protocol!();