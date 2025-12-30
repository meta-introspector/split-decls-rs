// Generated macro for generate_case_mapping (function)
macro_rules! Depcrate_case_mappinggenerate_case_mapping {
() => {
// Module: crate::case_mapping
// Provides: {"generate_case_mapping"}
// Dependencies: {}
pub (crate) fn generate_case_mapping (data : & UnicodeData) -> (String , [usize ; 2]) { let mut file = String :: new () ; write ! (file , "const INDEX_MASK: u32 = 0x{INDEX_MASK:x};") . unwrap () ; file . push_str ("\n\n") ; file . push_str (HEADER . trim_start ()) ; file . push ('\n') ; let (lower_tables , lower_size) = generate_tables ("LOWER" , & data . to_lower) ; file . push_str (& lower_tables) ; file . push_str ("\n\n") ; let (upper_tables , upper_size) = generate_tables ("UPPER" , & data . to_upper) ; file . push_str (& upper_tables) ; (file , [lower_size , upper_size]) }
};
}
