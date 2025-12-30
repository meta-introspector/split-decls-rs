// Generated macro for rewrite_opt_lifetime (function)
macro_rules! Depcrate_itemsrewrite_opt_lifetime {
() => {
// Module: crate::items
// Provides: {"rewrite_opt_lifetime"}
// Dependencies: {}
fn rewrite_opt_lifetime (context : & RewriteContext < '_ > , lifetime : Option < ast :: Lifetime > ,) -> RewriteResult { let Some (l) = lifetime else { return Ok (String :: new ()) ; } ; let mut result = l . rewrite_result (context , Shape :: legacy (context . config . max_width () , Indent :: empty ()) ,) ? ; result . push (' ') ; Ok (result) }
};
}
