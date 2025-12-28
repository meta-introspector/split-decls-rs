macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl fmt :: Debug for Span { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut span = f . debug_struct ("Span") ; if let Some (meta) = self . meta { span . field ("name" , & meta . name ()) . field ("level" , & meta . level ()) . field ("target" , & meta . target ()) ; if let Some (ref inner) = self . inner { span . field ("id" , & inner . id ()) ; } else { span . field ("disabled" , & true) ; } if let Some (ref path) = meta . module_path () { span . field ("module_path" , & path) ; } if let Some (ref line) = meta . line () { span . field ("line" , & line) ; } if let Some (ref file) = meta . file () { span . field ("file" , & file) ; } } else { span . field ("none" , & true) ; } span . finish () } }
    };
}

impl_65!();