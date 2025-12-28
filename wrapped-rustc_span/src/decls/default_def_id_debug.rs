macro_rules! deps {
    () => {
        DefId!();
    };
}

macro_rules! default_def_id_debug {
    () => {
        deps!();
        pub fn default_def_id_debug (def_id : DefId , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DefId") . field ("krate" , & def_id . krate) . field ("index" , & def_id . index) . finish () }
    };
}

default_def_id_debug!();