macro_rules! deps {
    () => {
        Document!();
        InlineTable!();
        KeyMut!();
        Array!();
        Formatted!();
        Value!();
        TableLike!();
        ArrayOfTables!();
        DocumentMut!();
        Item!();
        Table!();
    };
}

macro_rules! VisitMut {
    () => {
        deps!();
        # [doc = " Document tree traversal to mutate an exclusive borrow of a document tree in-place."] # [doc = ""] # [doc = " See the [module documentation](self) for details."] pub trait VisitMut { fn visit_document_mut (& mut self , node : & mut DocumentMut) { visit_document_mut (self , node) ; } fn visit_item_mut (& mut self , node : & mut Item) { visit_item_mut (self , node) ; } fn visit_table_mut (& mut self , node : & mut Table) { visit_table_mut (self , node) ; } fn visit_inline_table_mut (& mut self , node : & mut InlineTable) { visit_inline_table_mut (self , node) ; } # [doc = " [`visit_table_mut`](Self::visit_table_mut) and"] # [doc = " [`visit_inline_table_mut`](Self::visit_inline_table_mut) both recurse into this method."] fn visit_table_like_mut (& mut self , node : & mut dyn TableLike) { visit_table_like_mut (self , node) ; } fn visit_table_like_kv_mut (& mut self , key : KeyMut < '_ > , node : & mut Item) { visit_table_like_kv_mut (self , key , node) ; } fn visit_array_mut (& mut self , node : & mut Array) { visit_array_mut (self , node) ; } fn visit_array_of_tables_mut (& mut self , node : & mut ArrayOfTables) { visit_array_of_tables_mut (self , node) ; } fn visit_value_mut (& mut self , node : & mut Value) { visit_value_mut (self , node) ; } fn visit_boolean_mut (& mut self , node : & mut Formatted < bool >) { visit_boolean_mut (self , node) ; } fn visit_datetime_mut (& mut self , node : & mut Formatted < Datetime >) { visit_datetime_mut (self , node) ; } fn visit_float_mut (& mut self , node : & mut Formatted < f64 >) { visit_float_mut (self , node) ; } fn visit_integer_mut (& mut self , node : & mut Formatted < i64 >) { visit_integer_mut (self , node) ; } fn visit_string_mut (& mut self , node : & mut Formatted < String >) { visit_string_mut (self , node) ; } }
    };
}

VisitMut!()