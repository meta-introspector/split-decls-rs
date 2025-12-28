macro_rules! ToInt {
    () => {
        # [doc = " A **type operator** for taking a concrete integer value from a type."] # [doc = ""] # [doc = " It returns arbitrary integer value without explicitly specifying the"] # [doc = " type. It is useful when you pass the values to methods that accept"] # [doc = " distinct types without runtime casting."] pub trait ToInt < T > { # [doc = " Method returning the concrete value for the type."] fn to_int () -> T ; # [doc = " The concrete value for the type. Can be used in `const` contexts."] const INT : T ; }
    };
}

ToInt!()