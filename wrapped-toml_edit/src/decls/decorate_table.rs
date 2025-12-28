macro_rules! deps {
    () => {
        Table!();
    };
}

macro_rules! decorate_table {
    () => {
        deps!();
        fn decorate_table (table : & mut Table) { use indexmap :: map :: MutableKeys ; for (mut key , value) in table . items . iter_mut2 () . filter (| (_ , value) | value . is_value ()) . map (| (key , value) | (key . as_mut () , value . as_value_mut () . unwrap ())) { key . leaf_decor_mut () . clear () ; key . dotted_decor_mut () . clear () ; value . decor_mut () . clear () ; } }
    };
}

decorate_table!();