macro_rules! deps {
    () => {
        FSETable!();
    };
}

macro_rules! default_of_table {
    () => {
        deps!();
        pub (crate) fn default_of_table () -> FSETable { build_table_from_probabilities (OF_DIST , 5) }
    };
}

default_of_table!();