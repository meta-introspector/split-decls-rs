// Generated macro for on_upgrade (function)
macro_rules! Depcrate_filters_wson_upgrade {
() => {
// Module: crate::filters::ws
// Provides: {"on_upgrade"}
// Dependencies: {}
fn on_upgrade () -> impl Filter < Extract = (Option < OnUpgrade > ,) , Error = Rejection > + Copy { filter_fn_one (| route | future :: ready (Ok (route . extensions_mut () . remove :: < OnUpgrade > ()))) }
};
}
