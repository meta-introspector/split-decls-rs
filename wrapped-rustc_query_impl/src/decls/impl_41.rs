macro_rules! deps {
    () => {
        SpecIntoSelfProfilingString!();
        QueryKeyStringBuilder!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl SpecIntoSelfProfilingString for LocalDefId { fn spec_to_self_profile_string (& self , builder : & mut QueryKeyStringBuilder < '_ , '_ >) -> StringId { builder . def_id_to_string_id (DefId { krate : LOCAL_CRATE , index : self . local_def_index }) } }
    };
}

impl_41!()