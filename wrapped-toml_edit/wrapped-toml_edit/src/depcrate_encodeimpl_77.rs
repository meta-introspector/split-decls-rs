// Generated macro for impl_77 (impl)
macro_rules! Depcrate_encodeimpl_77 {
() => {
// Module: crate::encode
// Provides: {"impl_77"}
// Dependencies: {}
impl Display for DocumentMut { fn fmt (& self , f : & mut Formatter < '_ >) -> Result { let decor = self . decor () ; decor . prefix_encode (f , None , DEFAULT_ROOT_DECOR . 0) ? ; let mut path = Vec :: new () ; let mut last_position = 0 ; let mut tables = Vec :: new () ; visit_nested_tables (self . as_table () , & mut path , false , & mut | t , p , is_array | { if let Some (pos) = t . position () { last_position = pos ; } tables . push ((last_position , t , p . clone () , is_array)) ; Ok (()) }) . unwrap () ; tables . sort_by_key (| & (id , _ , _ , _) | id) ; let mut first_table = true ; for (_ , table , path , is_array) in tables { visit_table (f , None , table , & path , is_array , & mut first_table) ? ; } decor . suffix_encode (f , None , DEFAULT_ROOT_DECOR . 1) ? ; self . trailing () . encode_with_default (f , None , "") } }
};
}
