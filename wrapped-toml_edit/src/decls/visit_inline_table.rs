macro_rules! deps {
    () => {
        InlineTable!();
        Visit!();
    };
}

macro_rules! visit_inline_table {
    () => {
        deps!();
        pub fn visit_inline_table < 'doc , V > (v : & mut V , node : & 'doc InlineTable) where V : Visit < 'doc > + ? Sized , { v . visit_table_like (node) ; }
    };
}

visit_inline_table!();