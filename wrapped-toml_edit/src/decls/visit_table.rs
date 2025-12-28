macro_rules! deps {
    () => {
        Table!();
        Visit!();
    };
}

macro_rules! visit_table {
    () => {
        deps!();
        pub fn visit_table < 'doc , V > (v : & mut V , node : & 'doc Table) where V : Visit < 'doc > + ? Sized , { v . visit_table_like (node) ; }
    };
}

visit_table!()