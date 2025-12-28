macro_rules! deps {
    () => {
        ArrayOfTables!();
        Item!();
        Table!();
        VisitMut!();
        Value!();
    };
}

macro_rules! visit_item_mut {
    () => {
        deps!();
        pub fn visit_item_mut < V > (v : & mut V , node : & mut Item) where V : VisitMut + ? Sized , { match node { Item :: None => { } Item :: Value (value) => v . visit_value_mut (value) , Item :: Table (table) => v . visit_table_mut (table) , Item :: ArrayOfTables (array) => v . visit_array_of_tables_mut (array) , } }
    };
}

visit_item_mut!();