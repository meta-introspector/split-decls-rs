macro_rules! deps {
    () => {
        InlineTable!();
        VisitMut!();
    };
}

macro_rules! visit_inline_table_mut {
    () => {
        deps!();
        pub fn visit_inline_table_mut < V > (v : & mut V , node : & mut InlineTable) where V : VisitMut + ? Sized , { v . visit_table_like_mut (node) ; }
    };
}

visit_inline_table_mut!();