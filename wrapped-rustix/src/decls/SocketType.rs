macro_rules! deps {
    () => {
        RawSocketType!();
    };
}

macro_rules! SocketType {
    () => {
        deps!();
        # [doc = " `SOCK_*` constants for use with [`socket`]."] # [doc = ""] # [doc = " [`socket`]: crate::net::socket()"] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] # [repr (transparent)] pub struct SocketType (pub (crate) RawSocketType) ;
    };
}

SocketType!();