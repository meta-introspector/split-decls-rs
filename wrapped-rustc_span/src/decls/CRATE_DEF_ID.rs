macro_rules! deps {
    () => {
        LocalDefId!();
    };
}

macro_rules! CRATE_DEF_ID {
    () => {
        deps!();
        pub const CRATE_DEF_ID : LocalDefId = LocalDefId { local_def_index : CRATE_DEF_INDEX } ;
    };
}

CRATE_DEF_ID!()