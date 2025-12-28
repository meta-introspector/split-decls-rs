macro_rules! macro_658 {
    () => {
        bitflags ! { # [doc = " `SOCK_*` constants for use with [`socket_with`], [`accept_with`] and"] # [doc = " [`acceptfrom_with`]."] # [doc = ""] # [doc = " [`socket_with`]: crate::net::socket_with"] # [doc = " [`accept_with`]: crate::net::accept_with"] # [doc = " [`acceptfrom_with`]: crate::net::acceptfrom_with"] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct SocketFlags : ffi :: c_uint { # [doc = " `SOCK_NONBLOCK`"] # [cfg (not (any (apple , windows , target_os = "aix" , target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "nto" , target_os = "vita" ,)))] const NONBLOCK = bitcast ! (c :: SOCK_NONBLOCK) ; # [doc = " `SOCK_CLOEXEC`"] # [cfg (not (any (apple , windows , target_os = "aix" , target_os = "haiku")))] const CLOEXEC = bitcast ! (c :: SOCK_CLOEXEC) ; } }
    };
}

macro_658!()