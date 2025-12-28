macro_rules! deps {
    () => {
        HuffmanTable!();
    };
}

macro_rules! counts {
    () => {
        deps!();
        # [test] fn counts () { let counts = & [3 , 0 , 4 , 1 , 5] ; let table = HuffmanTable :: build_from_counts (counts) . codes ; assert_eq ! (table [1] . 1 , 0) ; assert ! (table [3] . 1 >= table [0] . 1) ; assert ! (table [0] . 1 >= table [2] . 1) ; assert ! (table [2] . 1 >= table [4] . 1) ; let counts = & [3 , 0 , 4 , 0 , 7 , 2 , 2 , 2 , 0 , 2 , 2 , 1 , 5] ; let table = HuffmanTable :: build_from_counts (counts) . codes ; assert_eq ! (table [1] . 1 , 0) ; assert_eq ! (table [3] . 1 , 0) ; assert_eq ! (table [8] . 1 , 0) ; assert ! (table [11] . 1 >= table [5] . 1) ; assert ! (table [5] . 1 >= table [6] . 1) ; assert ! (table [6] . 1 >= table [7] . 1) ; assert ! (table [7] . 1 >= table [9] . 1) ; assert ! (table [9] . 1 >= table [10] . 1) ; assert ! (table [10] . 1 >= table [0] . 1) ; assert ! (table [0] . 1 >= table [2] . 1) ; assert ! (table [2] . 1 >= table [12] . 1) ; assert ! (table [12] . 1 >= table [4] . 1) ; }
    };
}

counts!()