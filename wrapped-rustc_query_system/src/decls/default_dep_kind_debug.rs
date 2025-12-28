macro_rules! deps {
    () => {
        DepKind!();
    };
}

macro_rules! default_dep_kind_debug {
    () => {
        deps!();
        pub fn default_dep_kind_debug (kind : DepKind , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DepKind") . field ("variant" , & kind . variant) . finish () }
    };
}

default_dep_kind_debug!()