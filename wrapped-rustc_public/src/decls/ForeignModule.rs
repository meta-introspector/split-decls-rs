macro_rules! deps {
    () => {
        Abi!();
    };
}

macro_rules! ForeignModule {
    () => {
        deps!();
        pub struct ForeignModule { pub def_id : ForeignModuleDef , pub abi : Abi , }
    };
}

ForeignModule!()