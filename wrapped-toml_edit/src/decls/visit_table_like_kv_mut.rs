macro_rules! deps {
    () => {
        KeyMut!();
        Item!();
        VisitMut!();
    };
}

macro_rules! visit_table_like_kv_mut {
    () => {
        deps!();
        pub fn visit_table_like_kv_mut < V > (v : & mut V , _key : KeyMut < '_ > , node : & mut Item) where V : VisitMut + ? Sized , { v . visit_item_mut (node) ; }
    };
}

visit_table_like_kv_mut!();