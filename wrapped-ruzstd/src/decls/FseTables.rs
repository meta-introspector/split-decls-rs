macro_rules! deps {
    () => {
        FSETable!();
    };
}

macro_rules! FseTables {
    () => {
        deps!();
        pub (crate) struct FseTables { pub (crate) ll_default : FSETable , pub (crate) ll_previous : Option < FSETable > , pub (crate) ml_default : FSETable , pub (crate) ml_previous : Option < FSETable > , pub (crate) of_default : FSETable , pub (crate) of_previous : Option < FSETable > , }
    };
}

FseTables!()