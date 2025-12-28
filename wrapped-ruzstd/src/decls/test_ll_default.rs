macro_rules! deps {
    () => {
        FSETable!();
    };
}

macro_rules! test_ll_default {
    () => {
        deps!();
        # [test] fn test_ll_default () { let mut table = crate :: fse :: FSETable :: new (MAX_LITERAL_LENGTH_CODE) ; table . build_from_probabilities (LL_DEFAULT_ACC_LOG , & Vec :: from (& LITERALS_LENGTH_DEFAULT_DISTRIBUTION [..]) ,) . unwrap () ; # [cfg (feature = "std")] for idx in 0 .. table . decode . len () { std :: println ! ("{:3}: {:3} {:3} {:3}" , idx , table . decode [idx] . symbol , table . decode [idx] . num_bits , table . decode [idx] . base_line) ; } assert ! (table . decode . len () == 64) ; assert ! (table . decode [0] . symbol == 0) ; assert ! (table . decode [0] . num_bits == 4) ; assert ! (table . decode [0] . base_line == 0) ; assert ! (table . decode [19] . symbol == 27) ; assert ! (table . decode [19] . num_bits == 6) ; assert ! (table . decode [19] . base_line == 0) ; assert ! (table . decode [39] . symbol == 25) ; assert ! (table . decode [39] . num_bits == 4) ; assert ! (table . decode [39] . base_line == 16) ; assert ! (table . decode [60] . symbol == 35) ; assert ! (table . decode [60] . num_bits == 6) ; assert ! (table . decode [60] . base_line == 0) ; assert ! (table . decode [59] . symbol == 24) ; assert ! (table . decode [59] . num_bits == 5) ; assert ! (table . decode [59] . base_line == 32) ; }
    };
}

test_ll_default!()