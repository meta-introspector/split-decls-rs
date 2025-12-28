macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! insert_toml {
    () => {
        deps!();
        pub fn insert_toml (root : & mut Value , path : & [& str] , value : Value) { * traverse (root , path) = value ; }
    };
}

insert_toml!()