macro_rules! deps {
    () => {
        RawAddressFamily!();
    };
}

macro_rules! AddressFamily {
    () => {
        deps!();
        # [doc = " `AF_*` constants for use with [`socket`], [`socket_with`], and"] # [doc = " [`socketpair`]."] # [doc = ""] # [doc = " [`socket`]: crate::net::socket()"] # [doc = " [`socket_with`]: crate::net::socket_with"] # [doc = " [`socketpair`]: crate::net::socketpair()"] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] # [repr (transparent)] pub struct AddressFamily (pub (crate) RawAddressFamily) ;
    };
}

AddressFamily!();