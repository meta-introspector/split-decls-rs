macro_rules! deps {
    () => {
        Socket!();
    };
}

macro_rules! macro_40 {
    () => {
        deps!();
        from ! (Socket , net :: UdpSocket) ;
    };
}

macro_40!();