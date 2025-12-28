macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl Debug for Span { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Span") . field ("id" , & self . 0) . field ("repr" , & with (| cx | cx . span_to_string (* self))) . finish () } }
    };
}

impl_321!()