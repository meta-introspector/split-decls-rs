// Generated macro for debug_unwrap (macro)
macro_rules! Depcrate_helpersdebug_unwrap {
() => {
// Module: crate::helpers
// Provides: {"debug_unwrap"}
// Dependencies: {}
macro_rules ! debug_unwrap { ($ expr : expr , return $ retval : expr , $ ($ arg : tt) +) => { match $ expr { Some (x) => x , None => { debug_assert ! (false , $ ($ arg) *) ; return $ retval ; } } } ; ($ expr : expr , return $ retval : expr) => { debug_unwrap ! ($ expr , return $ retval , "invalid trie") } ; ($ expr : expr , break , $ ($ arg : tt) +) => { match $ expr { Some (x) => x , None => { debug_assert ! (false , $ ($ arg) *) ; break ; } } } ; ($ expr : expr , break) => { debug_unwrap ! ($ expr , break , "invalid trie") } ; ($ expr : expr , $ ($ arg : tt) +) => { debug_unwrap ! ($ expr , return () , $ ($ arg) *) } ; ($ expr : expr) => { debug_unwrap ! ($ expr , return ()) } ; }
};
}
