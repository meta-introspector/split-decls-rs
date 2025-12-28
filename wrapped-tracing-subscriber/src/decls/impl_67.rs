macro_rules! deps {
    () => {
        DynFilterFn!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < S , F , R > fmt :: Debug for DynFilterFn < S , F , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut s = f . debug_struct ("DynFilterFn") ; s . field ("enabled" , & format_args ! ("{}" , type_name ::< F > ())) ; if self . register_callsite . is_some () { s . field ("register_callsite" , & format_args ! ("Some({})" , type_name ::< R > ()) ,) ; } else { s . field ("register_callsite" , & format_args ! ("None")) ; } s . field ("max_level_hint" , & self . max_level_hint) . finish () } }
    };
}

impl_67!();