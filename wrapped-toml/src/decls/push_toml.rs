macro_rules! deps {
    () => {
        Table!();
        Value!();
        Array!();
    };
}

macro_rules! push_toml {
    () => {
        deps!();
        pub fn push_toml (root : & mut Value , path : & [& str]) { let target = traverse (root , path) ; if ! target . is_array () { * target = Value :: Array (Array :: new ()) ; } target . as_array_mut () . unwrap () . push (Value :: Table (Table :: new ())) ; }
    };
}

push_toml!();