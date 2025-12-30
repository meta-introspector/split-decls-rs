// Generated macro for Variables (struct)
macro_rules! Depcrate_variablesVariables {
() => {
// Module: crate::variables
// Provides: {"Variables"}
// Dependencies: {}
# [doc = " Variable mapping and metadata."] pub struct Variables { # [doc = " Bidirectional mapping from user variables to global variables."] # [doc = ""] # [doc = " Initially this is the identity mapping. This ensures that in the non-assumptions setting the"] # [doc = " map from used user variables to global variables is the identity. This is a requirement for"] # [doc = " generating proofs in non-native formats. Those proofs are not aware of variable renaming,"] # [doc = " but are restricted to the non-incremental setting, so this works out."] # [doc = ""] # [doc = " This is also requried for native proofs, as they assume that the mapping during the initial"] # [doc = " load is the identity."] global_from_user : VarBiMap , # [doc = " Bidirectional mapping from global variables to user variables."] # [doc = ""] # [doc = " This starts with the empty mapping, so only used variables are allocated."] solver_from_global : VarBiMap , # [doc = " User variables that were explicitly hidden by the user."] user_freelist : HashSet < Var > , # [doc = " Global variables that can be recycled without increasing the global_watermark."] global_freelist : HashSet < Var > , # [doc = " Solver variables that are unused and can be recycled."] solver_freelist : HashSet < Var > , # [doc = " Variable metadata."] # [doc = ""] # [doc = " Indexed by global variable indices."] var_data : Vec < VarData > , }
};
}
