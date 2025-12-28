macro_rules! deps {
    () => {
        HuffmanTable!();
    };
}

macro_rules! from_data {
    () => {
        deps!();
        # [test] fn from_data () { let counts = & [3 , 0 , 4 , 1 , 5] ; let table = HuffmanTable :: build_from_counts (counts) . codes ; let data = & [0 , 2 , 4 , 4 , 0 , 3 , 2 , 2 , 0 , 2] ; let table2 = HuffmanTable :: build_from_data (data) . codes ; assert_eq ! (table , table2) ; }
    };
}

from_data!();