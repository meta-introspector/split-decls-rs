macro_rules! deps {
    () => {
        VisitMut!();
        Table!();
    };
}

macro_rules! visit_table_mut {
    () => {
        deps!();
        pub fn visit_table_mut < V > (v : & mut V , node : & mut Table) where V : VisitMut + ? Sized , { v . visit_table_like_mut (node) ; }
    };
}

visit_table_mut!();