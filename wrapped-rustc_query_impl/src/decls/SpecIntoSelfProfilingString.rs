macro_rules! deps {
    () => {
        QueryKeyStringBuilder!();
    };
}

macro_rules! SpecIntoSelfProfilingString {
    () => {
        deps!();
        # [rustc_specialization_trait] trait SpecIntoSelfProfilingString : Debug { fn spec_to_self_profile_string (& self , builder : & mut QueryKeyStringBuilder < '_ , '_ >) -> StringId ; }
    };
}

SpecIntoSelfProfilingString!()