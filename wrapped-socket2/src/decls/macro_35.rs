macro_rules! deps {
    () => {
        Socket!();
    };
}

macro_rules! macro_35 {
    () => {
        deps!();
        from ! (net :: TcpStream , Socket) ;
    };
}

macro_35!();