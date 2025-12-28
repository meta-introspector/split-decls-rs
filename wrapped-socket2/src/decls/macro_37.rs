macro_rules! deps {
    () => {
        Socket!();
    };
}

macro_rules! macro_37 {
    () => {
        deps!();
        from ! (net :: UdpSocket , Socket) ;
    };
}

macro_37!();