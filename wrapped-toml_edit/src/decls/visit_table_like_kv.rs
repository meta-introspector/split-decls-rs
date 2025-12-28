macro_rules! deps {
    () => {
        Visit!();
        Item!();
    };
}

macro_rules! visit_table_like_kv {
    () => {
        deps!();
        pub fn visit_table_like_kv < 'doc , V > (v : & mut V , _key : & 'doc str , node : & 'doc Item) where V : Visit < 'doc > + ? Sized , { v . visit_item (node) ; }
    };
}

visit_table_like_kv!()