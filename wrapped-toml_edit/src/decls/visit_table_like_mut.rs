macro_rules! deps {
    () => {
        TableLike!();
        VisitMut!();
    };
}

macro_rules! visit_table_like_mut {
    () => {
        deps!();
        pub fn visit_table_like_mut < V > (v : & mut V , node : & mut dyn TableLike) where V : VisitMut + ? Sized , { for (key , item) in node . iter_mut () { v . visit_table_like_kv_mut (key , item) ; } }
    };
}

visit_table_like_mut!()