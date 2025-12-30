// Generated macro for shape_from_rhs_tactic (function)
macro_rules! Depcrate_exprshape_from_rhs_tactic {
() => {
// Module: crate::expr
// Provides: {"shape_from_rhs_tactic"}
// Dependencies: {}
fn shape_from_rhs_tactic (context : & RewriteContext < '_ > , shape : Shape , rhs_tactic : RhsTactics ,) -> Option < Shape > { match rhs_tactic { RhsTactics :: ForceNextLineWithoutIndent => shape . with_max_width (context . config) . sub_width_opt (shape . indent . width ()) , RhsTactics :: Default | RhsTactics :: AllowOverflow => { Shape :: indented (shape . indent . block_indent (context . config) , context . config) . sub_width_opt (shape . rhs_overhead (context . config)) } } }
};
}
