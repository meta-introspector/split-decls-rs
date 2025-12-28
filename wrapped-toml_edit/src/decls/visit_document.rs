macro_rules! deps {
    () => {
        DocumentMut!();
        Visit!();
    };
}

macro_rules! visit_document {
    () => {
        deps!();
        pub fn visit_document < 'doc , V > (v : & mut V , node : & 'doc DocumentMut) where V : Visit < 'doc > + ? Sized , { v . visit_table (node . as_table ()) ; }
    };
}

visit_document!();