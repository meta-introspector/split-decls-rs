// Generated macro for from_data (function)
macro_rules! Depcrate_huff0_huff0_encoderfrom_data {
() => {
// Module: crate::huff0::huff0_encoder
// Provides: {"from_data"}
// Dependencies: {}
# [test] fn from_data () { let counts = & [3 , 0 , 4 , 1 , 5] ; let table = HuffmanTable :: build_from_counts (counts) . codes ; let data = & [0 , 2 , 4 , 4 , 0 , 3 , 2 , 2 , 0 , 2] ; let table2 = HuffmanTable :: build_from_data (data) . codes ; assert_eq ! (table , table2) ; }
};
}
