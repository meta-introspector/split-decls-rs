macro_rules! deps {
    () => {
        Details!();
    };
}

macro_rules! DETAILS {
    () => {
        deps!();
        # [cfg (windows)] const DETAILS : & [Details] = & [s ! (SIGABRT , Term) , s ! (SIGFPE , Term) , s ! (SIGILL , Term) , s ! (SIGINT , Term) , s ! (SIGSEGV , Term) , s ! (SIGTERM , Term) ,] ;
    };
}

DETAILS!()