macro_rules! deps {
    () => {
        Socket!();
    };
}

macro_rules! macro_36 {
    () => {
        deps!();
        from ! (net :: TcpListener , Socket) ;
    };
}

macro_36!();