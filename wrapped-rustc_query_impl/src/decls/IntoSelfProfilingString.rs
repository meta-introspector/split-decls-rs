macro_rules! deps {
    () => {
        QueryKeyStringBuilder!();
    };
}

macro_rules! IntoSelfProfilingString {
    () => {
        deps!();
        trait IntoSelfProfilingString { fn to_self_profile_string (& self , builder : & mut QueryKeyStringBuilder < '_ , '_ >) -> StringId ; }
    };
}

IntoSelfProfilingString!();