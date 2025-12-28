macro_rules! deps {
    () => {
        ArrayOfTables!();
        VisitMut!();
    };
}

macro_rules! visit_array_of_tables_mut {
    () => {
        deps!();
        pub fn visit_array_of_tables_mut < V > (v : & mut V , node : & mut ArrayOfTables) where V : VisitMut + ? Sized , { for table in node . iter_mut () { v . visit_table_mut (table) ; } }
    };
}

visit_array_of_tables_mut!();