macro_rules! deps {
    () => {
        DepKind!();
    };
}

macro_rules! Stat {
    () => {
        deps!();
        struct Stat { kind : DepKind , node_counter : u64 , edge_counter : u64 , }
    };
}

Stat!();