macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl fmt :: Debug for Span { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fn fallback (span : Span , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Span") . field ("lo" , & span . lo ()) . field ("hi" , & span . hi ()) . field ("ctxt" , & span . ctxt ()) . finish () } if SESSION_GLOBALS . is_set () { with_session_globals (| session_globals | { if let Some (source_map) = & session_globals . source_map { write ! (f , "{} ({:?})" , source_map . span_to_diagnostic_string (* self) , self . ctxt ()) } else { fallback (* self , f) } }) } else { fallback (* self , f) } } }
    };
}

impl_289!()