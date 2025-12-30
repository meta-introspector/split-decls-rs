// Generated macro for Server (struct)
macro_rules! Depcrate_serverServer {
() => {
// Module: crate::server
// Provides: {"Server"}
// Dependencies: {}
# [doc = " A warp Server ready to filter requests."] # [doc = ""] # [doc = " Construct this type using [`serve()`]."] # [doc = ""] # [doc = " # Unnameable"] # [doc = ""] # [doc = " This type is publicly available in the docs only."] # [doc = ""] # [doc = " It is not otherwise nameable, since it is a builder type using typestate"] # [doc = " to allow for ergonomic configuration."] # [derive (Debug)] pub struct Server < F , A , R > { acceptor : A , filter : F , pipeline : bool , runner : R , }
};
}
