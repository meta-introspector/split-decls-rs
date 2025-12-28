macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_376 {
    () => {
        deps!();
        impl State { fn contains (& self , idx : usize) -> bool { self . baseline <= idx && self . last_index >= idx } }
    };
}

impl_376!()