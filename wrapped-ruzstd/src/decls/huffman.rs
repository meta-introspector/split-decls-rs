macro_rules! deps {
    () => {
        HuffmanTable!();
    };
}

macro_rules! huffman {
    () => {
        deps!();
        # [test] fn huffman () { let table = HuffmanTable :: build_from_weights (& [2 , 2 , 2 , 1 , 1]) ; assert_eq ! (table . codes [0] , (1 , 2)) ; assert_eq ! (table . codes [1] , (2 , 2)) ; assert_eq ! (table . codes [2] , (3 , 2)) ; assert_eq ! (table . codes [3] , (0 , 3)) ; assert_eq ! (table . codes [4] , (1 , 3)) ; let table = HuffmanTable :: build_from_weights (& [4 , 3 , 2 , 0 , 1 , 1]) ; assert_eq ! (table . codes [0] , (1 , 1)) ; assert_eq ! (table . codes [1] , (1 , 2)) ; assert_eq ! (table . codes [2] , (1 , 3)) ; assert_eq ! (table . codes [3] , (0 , 0)) ; assert_eq ! (table . codes [4] , (0 , 4)) ; assert_eq ! (table . codes [5] , (1 , 4)) ; }
    };
}

huffman!()