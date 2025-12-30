// Generated macro for ListTactic (enum)
macro_rules! Depcrate_config_listsListTactic {
() => {
// Module: crate::config::lists
// Provides: {"ListTactic"}
// Dependencies: {}
# [doc = " Formatting tactic for lists. This will be cast down to a"] # [doc = " `DefinitiveListTactic` depending on the number and length of the items and"] # [doc = " their comments."] # [config_type] pub enum ListTactic { # [doc = " One item per row."] Vertical , # [doc = " All items on one row."] Horizontal , # [doc = " Try Horizontal layout, if that fails then vertical."] HorizontalVertical , # [doc = " HorizontalVertical with a soft limit of n characters."] LimitedHorizontalVertical (usize) , # [doc = " Pack as many items as possible per row over (possibly) many rows."] Mixed , }
};
}
