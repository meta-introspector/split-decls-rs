// Generated macro for Variables (struct)
macro_rules! Depcrate_variablesVariables {
() => {
// Module: crate::variables
// Provides: {"Variables"}
// Dependencies: {}
# [derive (Default)] pub struct Variables { # [doc = " Information about literals in the current formula."] pub lit_data : Vec < LitData > , # [doc = " Information about variables in the current formula."] pub var_data : Vec < VarData > , # [doc = " User var names in use."] # [doc = ""] # [doc = " This is used to check for colliding mappings which are not allowed."] used_user_vars : HashSet < Var > , }
};
}
