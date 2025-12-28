macro_rules! deps {
    () => {
        Crate!();
        CrateNum!();
    };
}

macro_rules! crate_num {
    () => {
        deps!();
        pub fn crate_num (item : & crate :: Crate) -> CrateNum { item . id . into () }
    };
}

crate_num!();