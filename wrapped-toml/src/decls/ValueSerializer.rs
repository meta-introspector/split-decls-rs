macro_rules! deps {
    () => {
        Style!();
    };
}

macro_rules! ValueSerializer {
    () => {
        deps!();
        # [doc = " Serialization for TOML [values][crate::Value]."] # [doc = ""] # [doc = " This structure implements serialization support for TOML to serialize an"] # [doc = " arbitrary type to TOML. Note that the TOML format does not support all"] # [doc = " datatypes in Rust, such as enums, tuples, and tuple structs. These types"] # [doc = " will generate an error when serialized."] # [doc = ""] # [doc = " Currently a serializer always writes its output to an in-memory `String`,"] # [doc = " which is passed in when creating the serializer itself."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde::Serialize;"] # [doc = ""] # [doc = " #[derive(Serialize)]"] # [doc = " struct Config {"] # [doc = "     database: Database,"] # [doc = " }"] # [doc = ""] # [doc = " #[derive(Serialize)]"] # [doc = " struct Database {"] # [doc = "     ip: String,"] # [doc = "     port: Vec<u16>,"] # [doc = "     connection_max: u32,"] # [doc = "     enabled: bool,"] # [doc = " }"] # [doc = ""] # [doc = " let config = Config {"] # [doc = "     database: Database {"] # [doc = "         ip: \"192.168.1.1\".to_string(),"] # [doc = "         port: vec![8001, 8002, 8003],"] # [doc = "         connection_max: 5000,"] # [doc = "         enabled: false,"] # [doc = "     },"] # [doc = " };"] # [doc = ""] # [doc = " let mut value = String::new();"] # [doc = " serde::Serialize::serialize("] # [doc = "     &config,"] # [doc = "     toml::ser::ValueSerializer::new(&mut value)"] # [doc = " ).unwrap();"] # [doc = " println!(\"{}\", value)"] # [doc = " ```"] pub struct ValueSerializer < 'd > { dst : & 'd mut String , style : Style , }
    };
}

ValueSerializer!()