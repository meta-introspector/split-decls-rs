macro_rules! deps {
    () => {
        ArrayOfTables!();
        Value!();
        Table!();
    };
}

macro_rules! Item {
    () => {
        deps!();
        # [doc = " Type representing either a value, a table, an array of tables, or none."] # [derive (Debug , Default)] pub enum Item { # [doc = " Type representing none."] # [default] None , # [doc = " Type representing value."] Value (Value) , # [doc = " Type representing table."] Table (Table) , # [doc = " Type representing array of tables."] ArrayOfTables (ArrayOfTables) , }
    };
}

Item!();