macro_rules! deps {
    () => {
        FileAstId!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < N > PartialEq for FileAstId < N > { fn eq (& self , other : & Self) -> bool { self . raw == other . raw } }
    };
}

impl_17!()