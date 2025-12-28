macro_rules! deps {
    () => {
        FSETable!();
    };
}

macro_rules! default_ml_table {
    () => {
        deps!();
        pub (crate) fn default_ml_table () -> FSETable { build_table_from_probabilities (ML_DIST , 6) }
    };
}

default_ml_table!()