// Generated macro for get_tactics (function)
macro_rules! Depcrate_typesget_tactics {
() => {
// Module: crate::types
// Provides: {"get_tactics"}
// Dependencies: {}
fn get_tactics (item_vec : & [ListItem] , output : & str , shape : Shape) -> DefinitiveListTactic { if output . contains ('\n') { DefinitiveListTactic :: Vertical } else { definitive_tactic (item_vec , ListTactic :: HorizontalVertical , Separator :: Comma , shape . width . saturating_sub (2 + output . len ()) ,) } }
};
}
