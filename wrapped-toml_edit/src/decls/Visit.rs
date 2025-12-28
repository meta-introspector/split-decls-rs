macro_rules! deps {
    () => {
        Value!();
        Formatted!();
        InlineTable!();
        Document!();
        Table!();
        Array!();
        TableLike!();
        DocumentMut!();
        Item!();
        ArrayOfTables!();
    };
}

macro_rules! Visit {
    () => {
        deps!();
        # [doc = " Document tree traversal to mutate an exclusive borrow of a document tree in-place."] # [doc = ""] # [doc = " See the [module documentation](self) for details."] pub trait Visit < 'doc > { fn visit_document (& mut self , node : & 'doc DocumentMut) { visit_document (self , node) ; } fn visit_item (& mut self , node : & 'doc Item) { visit_item (self , node) ; } fn visit_table (& mut self , node : & 'doc Table) { visit_table (self , node) ; } fn visit_inline_table (& mut self , node : & 'doc InlineTable) { visit_inline_table (self , node) ; } fn visit_table_like (& mut self , node : & 'doc dyn TableLike) { visit_table_like (self , node) ; } fn visit_table_like_kv (& mut self , key : & 'doc str , node : & 'doc Item) { visit_table_like_kv (self , key , node) ; } fn visit_array (& mut self , node : & 'doc Array) { visit_array (self , node) ; } fn visit_array_of_tables (& mut self , node : & 'doc ArrayOfTables) { visit_array_of_tables (self , node) ; } fn visit_value (& mut self , node : & 'doc Value) { visit_value (self , node) ; } fn visit_boolean (& mut self , node : & 'doc Formatted < bool >) { visit_boolean (self , node) ; } fn visit_datetime (& mut self , node : & 'doc Formatted < Datetime >) { visit_datetime (self , node) ; } fn visit_float (& mut self , node : & 'doc Formatted < f64 >) { visit_float (self , node) ; } fn visit_integer (& mut self , node : & 'doc Formatted < i64 >) { visit_integer (self , node) ; } fn visit_string (& mut self , node : & 'doc Formatted < String >) { visit_string (self , node) ; } }
    };
}

Visit!();