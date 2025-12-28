macro_rules! deps {
    () => {
        FilterFn!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < F > fmt :: Debug for FilterFn < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FilterFn") . field ("enabled" , & format_args ! ("{}" , type_name ::< F > ())) . field ("max_level_hint" , & self . max_level_hint) . finish () } }
    };
}

impl_62!();