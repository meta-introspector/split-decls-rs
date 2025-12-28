macro_rules! deps {
    () => {
        ArrayOfTablesIterMut!();
        Item!();
        ArrayOfTablesIter!();
        ArrayOfTables!();
        Table!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl ArrayOfTables { # [doc = " Returns an iterator over tables."] pub fn iter (& self) -> ArrayOfTablesIter < '_ > { Box :: new (self . values . iter () . filter_map (Item :: as_table)) } # [doc = " Returns an iterator over tables."] pub fn iter_mut (& mut self) -> ArrayOfTablesIterMut < '_ > { Box :: new (self . values . iter_mut () . filter_map (Item :: as_table_mut)) } # [doc = " Returns the length of the underlying Vec."] # [doc = " To get the actual number of items use `a.iter().count()`."] pub fn len (& self) -> usize { self . values . len () } # [doc = " Returns true if `self.len() == 0`."] pub fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Removes all the tables."] pub fn clear (& mut self) { self . values . clear () ; } # [doc = " Returns an optional reference to the table."] pub fn get (& self , index : usize) -> Option < & Table > { self . values . get (index) . and_then (Item :: as_table) } # [doc = " Returns an optional mutable reference to the table."] pub fn get_mut (& mut self , index : usize) -> Option < & mut Table > { self . values . get_mut (index) . and_then (Item :: as_table_mut) } # [doc = " Appends a table to the array."] pub fn push (& mut self , table : Table) { self . values . push (Item :: Table (table)) ; } # [doc = " Removes a table with the given index."] pub fn remove (& mut self , index : usize) -> Table { self . values . remove (index) . into_table () . expect ("cannot have any other item in an array-of-tables") } # [doc = " Retains only the elements specified by the `keep` predicate."] # [doc = ""] # [doc = " In other words, remove all tables for which `keep(&table)` returns `false`."] # [doc = ""] # [doc = " This method operates in place, visiting each element exactly once in the"] # [doc = " original order, and preserves the order of the retained elements."] pub fn retain < F > (& mut self , mut keep : F) where F : FnMut (& Table) -> bool , { self . values . retain (| item | item . as_table () . map (& mut keep) . unwrap_or (false)) ; } }
    };
}

impl_17!()