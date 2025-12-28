macro_rules! deps {
    () => {
        Table!();
    };
}

macro_rules! required_table {
    () => {
        deps!();
        fn required_table (table : & Table) -> bool { if table . key . is_none () { ! table . body . is_empty () } else { table . array || ! table . body . is_empty () || ! table . has_children } }
    };
}

required_table!()