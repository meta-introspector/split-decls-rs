// Generated macro for assert_parse (macro)
macro_rules! Depcrate_macrosassert_parse {
() => {
// Module: crate::macros
// Provides: {"assert_parse"}
// Dependencies: {}
# [cfg (test)] macro_rules ! assert_parse (($ left : expr , $ right : expr) => { let res : $ crate :: error :: ModalResult < _ , $ crate :: error :: InputError < _ >> = $ left ; snapbox :: assert_data_eq ! (snapbox :: data :: ToDebug :: to_debug (& res) , $ right) ; } ;) ;
};
}
