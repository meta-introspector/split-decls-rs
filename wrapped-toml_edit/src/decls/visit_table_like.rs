macro_rules! deps {
    () => {
        Visit!();
        TableLike!();
    };
}

macro_rules! visit_table_like {
    () => {
        deps!();
        pub fn visit_table_like < 'doc , V > (v : & mut V , node : & 'doc dyn TableLike) where V : Visit < 'doc > + ? Sized , { for (key , item) in node . iter () { v . visit_table_like_kv (key , item) ; } }
    };
}

visit_table_like!();