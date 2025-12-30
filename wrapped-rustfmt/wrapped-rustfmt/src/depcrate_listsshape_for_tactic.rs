// Generated macro for shape_for_tactic (function)
macro_rules! Depcrate_listsshape_for_tactic {
() => {
// Module: crate::lists
// Provides: {"shape_for_tactic"}
// Dependencies: {}
pub (crate) fn shape_for_tactic (tactic : DefinitiveListTactic , h_shape : Option < Shape > , v_shape : Shape ,) -> Shape { match tactic { DefinitiveListTactic :: Horizontal => h_shape . unwrap () , _ => v_shape , } }
};
}
