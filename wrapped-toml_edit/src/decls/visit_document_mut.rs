macro_rules! deps {
    () => {
        VisitMut!();
        DocumentMut!();
    };
}

macro_rules! visit_document_mut {
    () => {
        deps!();
        pub fn visit_document_mut < V > (v : & mut V , node : & mut DocumentMut) where V : VisitMut + ? Sized , { v . visit_table_mut (node . as_table_mut ()) ; }
    };
}

visit_document_mut!();