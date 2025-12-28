macro_rules! deps {
    () => {
        QueryKeyStringBuilder!();
        SpecIntoSelfProfilingString!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl SpecIntoSelfProfilingString for DefId { fn spec_to_self_profile_string (& self , builder : & mut QueryKeyStringBuilder < '_ , '_ >) -> StringId { builder . def_id_to_string_id (* self) } }
    };
}

impl_38!()