// Generated macro for impl_724 (impl)
macro_rules! Depcrate_itemsimpl_724 {
() => {
// Module: crate::items
// Provides: {"impl_724"}
// Dependencies: {}
impl < 'a > Rewrite for TraitAliasBounds < 'a > { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { let generic_bounds_str = self . generic_bounds . rewrite_result (context , shape) ? ; let mut option = WhereClauseOption :: new (true , WhereClauseSpace :: None) ; option . allow_single_line () ; let where_str = rewrite_where_clause (context , & self . generics . where_clause . predicates , self . generics . where_clause . span , context . config . brace_style () , shape , false , ";" , None , self . generics . where_clause . span . lo () , option ,) ? ; let fits_single_line = ! generic_bounds_str . contains ('\n') && ! where_str . contains ('\n') && generic_bounds_str . len () + where_str . len () < shape . width ; let space = if generic_bounds_str . is_empty () || where_str . is_empty () { Cow :: from ("") } else if fits_single_line { Cow :: from (" ") } else { shape . indent . to_string_with_newline (context . config) } ; Ok (format ! ("{generic_bounds_str}{space}{where_str}")) } }
};
}
