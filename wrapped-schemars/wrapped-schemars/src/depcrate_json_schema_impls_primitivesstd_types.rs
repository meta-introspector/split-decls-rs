// Generated macro for std_types (module)
macro_rules! Depcrate_json_schema_impls_primitivesstd_types {
() => {
// Module: crate::json_schema_impls::primitives
// Provides: {"std_types"}
// Dependencies: {}
# [cfg (feature = "std")] mod std_types { use super :: * ; use std :: net :: { IpAddr , Ipv4Addr , Ipv6Addr , SocketAddr , SocketAddrV4 , SocketAddrV6 } ; use std :: path :: { Path , PathBuf } ; simple_impl ! (Path => "string") ; simple_impl ! (PathBuf => "string") ; simple_impl ! (Ipv4Addr => "string" , "ipv4") ; simple_impl ! (Ipv6Addr => "string" , "ipv6") ; simple_impl ! (IpAddr => "string" , "ip") ; simple_impl ! (SocketAddr => "string") ; simple_impl ! (SocketAddrV4 => "string") ; simple_impl ! (SocketAddrV6 => "string") ; }
};
}
