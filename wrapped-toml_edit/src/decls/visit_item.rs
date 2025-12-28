macro_rules! deps {
    () => {
        Value!();
        Visit!();
        Item!();
        Table!();
        ArrayOfTables!();
    };
}

macro_rules! visit_item {
    () => {
        deps!();
        pub fn visit_item < 'doc , V > (v : & mut V , node : & 'doc Item) where V : Visit < 'doc > + ? Sized , { match node { Item :: None => { } Item :: Value (value) => v . visit_value (value) , Item :: Table (table) => v . visit_table (table) , Item :: ArrayOfTables (array) => v . visit_array_of_tables (array) , } }
    };
}

visit_item!()