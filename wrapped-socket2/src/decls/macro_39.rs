macro_rules! deps {
    () => {
        Socket!();
    };
}

macro_rules! macro_39 {
    () => {
        deps!();
        from ! (Socket , net :: TcpListener) ;
    };
}

macro_39!()