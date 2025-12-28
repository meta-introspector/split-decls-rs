use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Network operations trait - 10 calls (0.3% of all syscalls)"] # [doc = " Strategy: Validation required for security"] pub trait NetworkOps : Send + Sync { fn tcp_connect (& self , addr : SocketAddr) -> IoResult < TcpStream > ; fn tcp_bind (& self , addr : SocketAddr) -> IoResult < std :: net :: TcpListener > ; }