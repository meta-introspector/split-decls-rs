macro_rules! ToJson {
    () => {
        pub trait ToJson { fn to_json (& self) -> Json ; }
    };
}

ToJson!()