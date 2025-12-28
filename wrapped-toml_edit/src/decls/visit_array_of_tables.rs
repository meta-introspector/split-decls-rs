macro_rules! deps {
    () => {
        ArrayOfTables!();
        Visit!();
    };
}

macro_rules! visit_array_of_tables {
    () => {
        deps!();
        pub fn visit_array_of_tables < 'doc , V > (v : & mut V , node : & 'doc ArrayOfTables) where V : Visit < 'doc > + ? Sized , { for table in node . iter () { v . visit_table (table) ; } }
    };
}

visit_array_of_tables!()