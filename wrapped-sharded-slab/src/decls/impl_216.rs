macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl PartialEq for State { fn eq (& self , other : & State) -> bool { self . id . eq (& other . id) } }
    };
}

impl_216!();