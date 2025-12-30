// Generated macro for to_string (function)
macro_rules! Depcrate_serto_string {
() => {
// Module: crate::ser
// Provides: {"to_string"}
// Dependencies: {}
# [doc = " Serialize the given data structure as a String of TOML."] # [doc = ""] # [doc = " Serialization can fail if `T`'s implementation of `Serialize` decides to"] # [doc = " fail, if `T` contains a map with non-string keys, or if `T` attempts to"] # [doc = " serialize an unsupported datatype such as an enum, tuple, or tuple struct."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde::Serialize;"] # [doc = ""] # [doc = " #[derive(Serialize)]"] # [doc = " struct Config {"] # [doc = "     database: Database,"] # [doc = " }"] # [doc = ""] # [doc = " #[derive(Serialize)]"] # [doc = " struct Database {"] # [doc = "     ip: String,"] # [doc = "     port: Vec<u16>,"] # [doc = "     connection_max: u32,"] # [doc = "     enabled: bool,"] # [doc = " }"] # [doc = ""] # [doc = " let config = Config {"] # [doc = "     database: Database {"] # [doc = "         ip: \"192.168.1.1\".to_string(),"] # [doc = "         port: vec![8001, 8002, 8003],"] # [doc = "         connection_max: 5000,"] # [doc = "         enabled: false,"] # [doc = "     },"] # [doc = " };"] # [doc = ""] # [doc = " let toml = toml_edit::ser::to_string(&config).unwrap();"] # [doc = " println!(\"{}\", toml)"] # [doc = " ```"] # [cfg (feature = "display")] pub fn to_string < T > (value : & T) -> Result < String , Error > where T : serde_core :: ser :: Serialize + ? Sized , { to_document (value) . map (| e | e . to_string ()) }
};
}
