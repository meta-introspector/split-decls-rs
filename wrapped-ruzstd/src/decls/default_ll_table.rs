macro_rules! deps {
    () => {
        FSETable!();
    };
}

macro_rules! default_ll_table {
    () => {
        deps!();
        pub (crate) fn default_ll_table () -> FSETable { build_table_from_probabilities (LL_DIST , 6) }
    };
}

default_ll_table!();