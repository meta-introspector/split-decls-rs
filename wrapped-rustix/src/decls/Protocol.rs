macro_rules! deps {
    () => {
        RawProtocol!();
    };
}

macro_rules! Protocol {
    () => {
        deps!();
        # [doc = " `IPPROTO_*` and other constants for use with [`socket`], [`socket_with`],"] # [doc = " and [`socketpair`] when a nondefault value is desired."] # [doc = ""] # [doc = " See the [`ipproto`], [`sysproto`], and [`netlink`] modules for possible"] # [doc = " values."] # [doc = ""] # [doc = " For the default values, such as `IPPROTO_IP` or `NETLINK_ROUTE`, pass"] # [doc = " `None` as the `protocol` argument in these functions."] # [doc = ""] # [doc = " [`socket`]: crate::net::socket()"] # [doc = " [`socket_with`]: crate::net::socket_with"] # [doc = " [`socketpair`]: crate::net::socketpair()"] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] # [repr (transparent)] # [doc (alias = "IPPROTO_IP")] # [doc (alias = "NETLINK_ROUTE")] pub struct Protocol (pub (crate) RawProtocol) ;
    };
}

Protocol!();