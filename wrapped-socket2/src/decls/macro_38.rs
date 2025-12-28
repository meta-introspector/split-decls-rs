macro_rules! deps {
    () => {
        Socket!();
    };
}

macro_rules! macro_38 {
    () => {
        deps!();
        from ! (Socket , net :: TcpStream) ;
    };
}

macro_38!();