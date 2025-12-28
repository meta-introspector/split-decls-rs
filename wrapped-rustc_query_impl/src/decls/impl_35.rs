macro_rules! deps {
    () => {
        IntoSelfProfilingString!();
        QueryKeyStringBuilder!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T : Debug > IntoSelfProfilingString for T { default fn to_self_profile_string (& self , builder : & mut QueryKeyStringBuilder < '_ , '_ > ,) -> StringId { let s = format ! ("{self:?}") ; builder . profiler . alloc_string (& s [..]) } }
    };
}

impl_35!();