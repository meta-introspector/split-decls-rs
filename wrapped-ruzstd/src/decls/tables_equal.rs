macro_rules! deps {
    () => {
        FSETable!();
    };
}

macro_rules! tables_equal {
    () => {
        deps!();
        # [test] fn tables_equal () { let probs = & [0 , 0 , - 1 , 3 , 2 , 2 , (1 << 6) - 8] ; let mut dec_table = FSETable :: new (255) ; dec_table . build_from_probabilities (6 , probs) . unwrap () ; let enc_table = fse_encoder :: build_table_from_probabilities (probs , 6) ; check_tables (& dec_table , & enc_table) ; }
    };
}

tables_equal!();