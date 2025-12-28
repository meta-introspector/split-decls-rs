macro_rules! deps {
    () => {
        SpecIntoSelfProfilingString!();
        QueryKeyStringBuilder!();
        IntoSelfProfilingString!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < T : SpecIntoSelfProfilingString > IntoSelfProfilingString for T { fn to_self_profile_string (& self , builder : & mut QueryKeyStringBuilder < '_ , '_ >) -> StringId { self . spec_to_self_profile_string (builder) } }
    };
}

impl_36!();